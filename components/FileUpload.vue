<template>
  <div class="group/file relative block w-full h-full overflow-auto rounded-lg p-10">
    <input ref="fileInputRef" type="file" class="hidden" @change="onFileChange" multiple accept="image/*,video/*" />

    <!-- Content -->
    <div class="flex flex-col items-center justify-center w-full h-full">
      <p class="relative z-20 font-sans text-base font-bold text-neutral-700 dark:text-neutral-300">Upload files</p>
      <p class="relative z-20 mt-2 font-sans text-base font-normal text-neutral-400 dark:text-neutral-400">
        Drag or drop your images and videos here or click to upload
      </p>

      <div class="relative mx-auto mt-10 w-full space-y-4 flex justify-center flex-col items-center">
        <!-- Loading Files (show while processing) -->
        <template v-for="(loadingFile, idx) in loadingFiles" :key="`loading-${idx}`">
          <FileLoadingSkeleton :file-name="loadingFile.name" :file-type="loadingFile.type" />
        </template>

        <!-- Render Loaded Image and Video Files -->
        <template v-for="(file, idx) in files" :key="`file-${idx}`">
          <!-- Image Files -->
          <FileImage
            v-if="file.type.startsWith('image/')"
            :file="file"
            :index="idx"
            @remove="removeFile(idx)"
            @updateOption="options => updateFileOption(idx, options)" />

          <!-- Video Files -->
          <FileVideo
            v-else-if="file.type.startsWith('video/')"
            :file="file"
            :index="idx"
            @remove="removeFile(idx)"
            @updateOption="options => updateFileOption(idx, options)" />
        </template>

        <template v-if="files.length">
          <UButton @click="handleClick" class="w-fit mt-4 py-2 rounded-md" leading-icon="lucide:circle-plus"> Add More Files </UButton>
        </template>

        <!-- Upload Area -->
        <template v-if="!files.length && !loadingFiles.length">
          <div
            @click="handleClick"
            @mouseenter="handleEnter"
            @mouseleave="handleLeave"
            :data-active="isActive"
            :class="{ 'pointer-events-none opacity-50': isProcessingFiles }"
            v-gsap.onState-active.fromTo="[
              { x: 0, y: 0, opacity: 1 },
              { x: 20, y: -20, opacity: 0.9, duration: 0.2, ease: 'ease-in' }
            ]"
            class="relative cursor-pointer z-40 mx-auto mt-4 flex h-32 w-full max-w-32 items-center justify-center rounded-md bg-primary-500 shadow-[0px_10px_50px_rgba(0,0,0,0.1)] group-hover/file:shadow-2xl">
            <Icon name="heroicons:arrow-up-tray-20-solid" class="text-white" size="20" />
          </div>

          <div
            class="absolute inset-0 z-30 mx-auto mt-4 flex h-32 w-full max-w-32 items-center justify-center rounded-md border border-dashed border-sky-400 bg-transparent transition-opacity"
            :class="{ 'opacity-100': isActive, 'opacity-0': !isActive }"></div>
        </template>

        <!-- Processing Status -->
        <div v-if="isProcessingFiles" class="text-center py-4">
          <p class="text-sm text-neutral-500 dark:text-neutral-400">
            Processing {{ loadingFiles.length }} file{{ loadingFiles.length > 1 ? 's' : '' }}...
          </p>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import type { HTMLAttributes } from 'vue';
import { cn } from '@/lib/utils';
import { ref, onMounted, onUnmounted, computed } from 'vue';
import { useFileStore, type FileWithPath } from '@/stores/fileStore';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';
import FileImage from '@/components/File/Image.vue';
import FileVideo from '@/components/File/Video.vue';
import FileLoadingSkeleton from '@/components/File/LoadingSkeleton.vue';
import { getFileExtension, getFileTypeFromExtension } from '@/utils/fileHelpers';
import type { MediaInfo } from '@/types/tauri';

interface FileUploadProps {
  class?: HTMLAttributes['class'];
}

defineProps<FileUploadProps>();

const emit = defineEmits<{
  (e: 'onChange', files: FileWithPath[]): void;
}>();

const fileInputRef = ref<HTMLInputElement | null>(null);
const isActive = ref<boolean>(false);
const isProcessingFiles = ref<boolean>(false);
const loadingFiles = ref<Array<{ name: string; type: string }>>([]);
const fileStore = useFileStore();

// Use the store's files instead of local state
const files = computed(() =>
  fileStore.selectedFiles.map(file => {
    // Ensure every file has options
    if (!file.options) {
      const isVideo = file.type.startsWith('video/');
      file.options = {
        format: isVideo ? 'mp4' : 'jpeg',
        width: file.width,
        height: file.height,
        maintainAspectRatio: true,
        destinationFolder: '',
        // Video specific defaults
        ...(isVideo && {
          duration: undefined, // Will be detected from video file
          fps: 30,
          keepFpsRatio: false
        })
      };
    }
    return file;
  })
);

onMounted(() => {
  console.log('FileUpload component mounted.');
});

onUnmounted(() => {
  console.log('FileUpload component unmounted.');
});

async function handleFileChange(newFiles: FileWithPath[]) {
  fileStore.addFiles(newFiles);
  emit('onChange', fileStore.selectedFiles);
}

async function processFiles(filePaths: string[]) {
  isProcessingFiles.value = true;

  // Add loading placeholders
  loadingFiles.value = filePaths.map(path => {
    const extension = getFileExtension(path);
    const fileType = getFileTypeFromExtension(extension);
    return {
      name: path.split(/[\\/]/).pop() || path,
      type: fileType
    };
  });

  try {
    // Process files individually - show results as soon as each is ready
    const processPromises = filePaths.map(async (path, index) => {
      try {
        const fileInfo = await getFileInfo(path);
        const extension = getFileExtension(path);
        const fileType = getFileTypeFromExtension(extension);
        const isVideo = fileType.startsWith('video/');

        // Get media info (dimensions, duration, fps) via backend
        let width, height, duration, fps;
        try {
          const mediaInfo = (await invoke('get_file_info', { filePath: path })) as MediaInfo;
          width = mediaInfo.width;
          height = mediaInfo.height;
          duration = mediaInfo.duration;
          fps = mediaInfo.fps;
        } catch (e) {
          console.warn('Failed to get media info from backend, falling back to basic info:', e);
          // Fallback to basic dimensions
          try {
            const dims = await invoke('get_image_dimensions', { path });
            if (Array.isArray(dims) && dims.length === 2) {
              width = dims[0];
              height = dims[1];
            } else if (typeof dims === 'object' && dims !== null && 'width' in dims && 'height' in dims) {
              width = dims.width;
              height = dims.height;
            }
          } catch (e2) {
            console.warn('Failed to get dimensions:', e2);
            width = fileInfo.width;
            height = fileInfo.height;
          }
        }

        const fileObject: FileWithPath = {
          name: path.split(/[\\/]/).pop() || path,
          size: fileInfo.size,
          type: fileType,
          lastModified: Date.now(),
          path: path,
          width,
          height,
          options: {
            format: isVideo ? 'mp4' : 'jpeg',
            width: width,
            height: height,
            maintainAspectRatio: true,
            destinationFolder: '',
            // Video specific defaults
            ...(isVideo && {
              duration: duration,
              durationMode: 'trim', // Default to trim mode
              fps: fps || 30,
              keepFpsRatio: false
            })
          }
        };

        // Remove the loading placeholder for this specific file
        loadingFiles.value = loadingFiles.value.filter(loadingFile => loadingFile.name !== fileObject.name);

        // Add the file immediately when it's ready
        await handleFileChange([fileObject]);

        return fileObject;
      } catch (error) {
        console.error(`Error processing file ${path}:`, error);
        // Remove the loading placeholder for this file even if it failed
        const fileName = path.split(/[\\/]/).pop() || path;
        loadingFiles.value = loadingFiles.value.filter(loadingFile => loadingFile.name !== fileName);
        return null;
      }
    });

    // Wait for all processing to complete (but files are already added individually)
    await Promise.allSettled(processPromises);
  } catch (error) {
    console.error('Error processing files:', error);
  } finally {
    isProcessingFiles.value = false;
    loadingFiles.value = [];
  }
}

function onFileChange(e: Event) {
  const input = e.target as HTMLInputElement;
  if (!input.files) return;

  isProcessingFiles.value = true;

  // Add loading placeholders for HTML5 files
  loadingFiles.value = Array.from(input.files).map(file => ({
    name: file.name,
    type: file.type
  }));

  // Convert File objects to our format and process individually
  const processPromises = Array.from(input.files).map(async (file, index) => {
    try {
      let dimensions: { width?: number; height?: number } = {};

      // Get dimensions based on file type
      if (file.type.startsWith('image/')) {
        dimensions = await getImageDimensions(file);
      } else if (file.type.startsWith('video/')) {
        dimensions = await getVideoDimensions(file);
      }

      const isVideo = file.type.startsWith('video/');

      const fileObject: FileWithPath = {
        name: file.name,
        size: file.size,
        type: file.type,
        lastModified: file.lastModified,
        path: '', // HTML5 files don't have paths
        width: dimensions.width,
        height: dimensions.height,
        options: {
          format: isVideo ? 'mp4' : 'jpeg',
          width: dimensions.width,
          height: dimensions.height,
          maintainAspectRatio: true,
          destinationFolder: '',
          // Video specific defaults
          ...(isVideo && {
            duration: undefined, // Cannot be detected from HTML5 file
            fps: 30,
            keepFpsRatio: false
          })
        }
      };

      // Remove loading placeholder for this specific file
      loadingFiles.value = loadingFiles.value.filter(loadingFile => loadingFile.name !== file.name);

      // Add the file immediately when it's ready
      await handleFileChange([fileObject]);

      return fileObject;
    } catch (error) {
      console.error(`Error processing HTML5 file ${file.name}:`, error);
      // Remove the loading placeholder for this file even if it failed
      loadingFiles.value = loadingFiles.value.filter(loadingFile => loadingFile.name !== file.name);
      return null;
    }
  });

  // Wait for all processing to complete (but files are already added individually)
  Promise.allSettled(processPromises)
    .then(() => {
      isProcessingFiles.value = false;
      loadingFiles.value = [];
    })
    .catch(error => {
      console.error('Error processing HTML5 files:', error);
      isProcessingFiles.value = false;
      loadingFiles.value = [];
    });
}

async function handleClick() {
  if (isProcessingFiles.value) return;

  try {
    const selected = await open({
      title: 'Select Image and Video Files',
      multiple: true,
      filters: [
        {
          name: 'Media Files',
          extensions: ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'tiff', 'gif', 'mp4', 'avi', 'mov', 'wmv', 'flv', 'webm', 'mkv']
        },
        {
          name: 'Image Files',
          extensions: ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'tiff', 'gif']
        },
        {
          name: 'Video Files',
          extensions: ['mp4', 'avi', 'mov', 'wmv', 'flv', 'webm', 'mkv']
        }
      ]
    });

    if (selected && Array.isArray(selected)) {
      await processFiles(selected);
    } else if (selected) {
      await processFiles([selected as string]);
    }
  } catch (error) {
    console.error('Error selecting files:', error);
    isProcessingFiles.value = false;
    loadingFiles.value = [];
  }
}

function handleEnter() {
  if (!isProcessingFiles.value) {
    console.log('Mouse Enter');
    isActive.value = true;
  }
}

function handleLeave() {
  isActive.value = false;
}

async function getFileInfo(filePath: string): Promise<{ size: number; width?: number; height?: number }> {
  try {
    // Use Tauri to get file stats for size and dimensions
    const stats = (await invoke('get_file_stats', { filePath })) as { size: number; width?: number; height?: number };
    return stats;
  } catch (error) {
    console.error('Error getting file info:', error);
    return { size: 0 };
  }
}

async function getImageDimensions(file: File): Promise<{ width?: number; height?: number }> {
  return new Promise(resolve => {
    const img = new Image();
    img.onload = () => {
      resolve({ width: img.naturalWidth, height: img.naturalHeight });
    };
    img.onerror = () => {
      resolve({});
    };
    img.src = URL.createObjectURL(file);
  });
}

async function getVideoDimensions(file: File): Promise<{ width?: number; height?: number }> {
  return new Promise(resolve => {
    const video = document.createElement('video');
    video.preload = 'metadata';
    video.onloadedmetadata = () => {
      resolve({ width: video.videoWidth, height: video.videoHeight });
      URL.revokeObjectURL(video.src);
    };
    video.onerror = () => {
      resolve({});
      URL.revokeObjectURL(video.src);
    };
    video.src = URL.createObjectURL(file);
  });
}

function removeFile(index: number): void {
  fileStore.removeFile(index);
}

// --- Per-file options logic ---
function updateFileOption(idx: number, options: any) {
  fileStore.updateFileOptions(idx, options);
}
</script>

<style scoped>
.group-hover\/file\:shadow-2xl:hover {
  box-shadow: 0px 10px 20px rgba(0, 0, 0, 0.25);
}

.transition-opacity {
  transition: opacity 0.3s ease;
}
</style>
