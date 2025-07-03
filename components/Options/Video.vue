<template>
  <div class="space-y-4">
    <!-- Format Selection -->
    <div>
      <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">Format</label>
      <USelect
        v-model="selectedFile.options!.format"
        value-key="id"
        :items="videoFormatOptions"
        @update:model-value="updateFileOptions({ format: $event })" />
    </div>

    <!-- Output Name -->
    <div>
      <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">Output Name</label>
      <UInput
        v-model="selectedFile.options!.outputName"
        @input="updateFileOptions({ outputName: selectedFile.options!.outputName })"
        placeholder="Enter output filename (without extension)"
        class="w-full" />
      <p class="text-xs text-neutral-500 dark:text-neutral-400 mt-1">File will be saved as: {{ getOutputFileName() }}</p>
    </div>

    <!-- Destination Folder -->
    <div>
      <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">Output Folder</label>
      <div class="flex items-center gap-2">
        <UInput v-model="selectedFile.options!.destinationFolder" placeholder="Choose output folder..." readonly class="flex-1" />
        <UButton @click="selectDestinationFolder" icon="i-heroicons-folder-20-solid" size="sm" color="neutral" variant="outline">
          Browse
        </UButton>
      </div>
      <p class="text-xs text-neutral-500 dark:text-neutral-400 mt-1">
        {{ selectedFile.options!.destinationFolder || 'Same as source file location' }}
      </p>
    </div>

    <!-- Dimensions -->
    <div>
      <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">Dimensions</label>
      <div class="flex items-center gap-2">
        <UInput
          type="number"
          v-model.number="selectedFile.options!.targetWidth"
          @change="onTargetWidthInput"
          placeholder="Width"
          class="flex-1" />
        <span class="text-neutral-500">×</span>
        <UInput
          type="number"
          v-model.number="selectedFile.options!.targetHeight"
          @change="onTargetHeightInput"
          placeholder="Height"
          class="flex-1" />
      </div>
      <div class="mt-2 space-y-1">
        <UCheckbox
          v-model="selectedFile.options!.maintainAspectRatio"
          @change="updateFileOptions({ maintainAspectRatio: selectedFile.options!.maintainAspectRatio })"
          label="Maintain aspect ratio" />
        <p
          class="text-xs text-neutral-500 dark:text-neutral-400"
          v-if="selectedFile.options!.originalWidth && selectedFile.options!.originalHeight"></p>
      </div>
    </div>

    <!-- Duration and FPS -->
    <div class="space-y-3">
      <div class="grid grid-cols-2 gap-2">
        <div>
          <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">Duration</label>
          <UInput
            type="text"
            v-model="durationDisplay"
            @input="onDurationInput"
            placeholder="MM:SS"
            title="Duration in MM:SS or HH:MM:SS format" />
        </div>
        <div>
          <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-2">FPS</label>
          <UInput type="number" v-model.number="selectedFile.options!.fps" @input="onFpsInput" placeholder="FPS" />
        </div>
      </div>
    </div>

    <!-- Audio Options -->
    <div class="space-y-2">
      <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300">Audio</label>
      <div>
        <UCheckbox
          v-model="selectedFile.options!.disableAudio"
          @change="updateFileOptions({ disableAudio: selectedFile.options!.disableAudio })"
          label="Remove audio from output" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useFileStore } from '@/stores/fileStore';
import { formatDuration, parseDuration } from '@/utils/fileHelpers';
import { open } from '@tauri-apps/plugin-dialog';

const fileStore = useFileStore();

// Props
interface Props {
  selectedFile: any;
}

const props = defineProps<Props>();

// Format options
const videoFormatOptions = [
  { label: 'MP4', id: 'mp4' },
  { label: 'AVI', id: 'avi' },
  { label: 'MOV', id: 'mov' },
  { label: 'WEBM', id: 'webm' },
  { label: 'MKV', id: 'mkv' }
];

// Duration display for videos
const durationDisplay = computed({
  get: () => {
    if (props.selectedFile?.options?.duration) {
      return formatDuration(props.selectedFile.options.duration);
    }
    return '';
  },
  set: (value: string) => {
    const seconds = parseDuration(value);
    if (seconds > 0 && fileStore.selectedFileIndex !== null) {
      fileStore.updateFileOptions(fileStore.selectedFileIndex, { duration: seconds });
    }
  }
});

// Update file options
function updateFileOptions(options: any) {
  if (fileStore.selectedFileIndex !== null) {
    fileStore.updateFileOptions(fileStore.selectedFileIndex, options);
  }
}

// Target width input handler with aspect ratio
function onTargetWidthInput() {
  if (!props.selectedFile?.options?.targetWidth || fileStore.selectedFileIndex === null) return;

  const options = props.selectedFile.options;
  const newWidth = options.targetWidth;

  if (options.maintainAspectRatio && options.originalAspectRatio && newWidth) {
    // width = height * aspectRatio  =>  height = width / aspectRatio
    const newHeight = newWidth / options.originalAspectRatio;
    console.log(newHeight, options.originalAspectRatio);
    fileStore.updateFileOptions(fileStore.selectedFileIndex, {
      targetWidth: newWidth,
      targetHeight: newHeight,
      width: newWidth,
      height: newHeight
    });
  } else {
    fileStore.updateFileOptions(fileStore.selectedFileIndex, {
      targetWidth: newWidth,
      width: newWidth
    });
  }
}

// Target height input handler with aspect ratio
function onTargetHeightInput() {
  if (!props.selectedFile?.options?.targetHeight || fileStore.selectedFileIndex === null) return;

  const options = props.selectedFile.options;
  const newHeight = options.targetHeight;

  if (options.maintainAspectRatio && options.originalAspectRatio && newHeight) {
    // height = width / aspectRatio  =>  width = height * aspectRatio
    const newWidth = Math.round(newHeight * options.originalAspectRatio);
    fileStore.updateFileOptions(fileStore.selectedFileIndex, {
      targetWidth: newWidth,
      targetHeight: newHeight,
      width: newWidth,
      height: newHeight
    });
  } else {
    fileStore.updateFileOptions(fileStore.selectedFileIndex, {
      targetHeight: newHeight,
      height: newHeight
    });
  }
}

// Duration input handler
function onDurationInput(event: Event) {
  const target = event.target as HTMLInputElement;
  const seconds = parseDuration(target.value);
  if (seconds > 0 && fileStore.selectedFileIndex !== null) {
    fileStore.updateFileOptions(fileStore.selectedFileIndex, { duration: seconds });
  }
}

// FPS input handler
function onFpsInput() {
  if (props.selectedFile?.options?.fps && fileStore.selectedFileIndex !== null) {
    fileStore.updateFileOptions(fileStore.selectedFileIndex, { fps: props.selectedFile.options.fps });
  }
}

// Select destination folder
async function selectDestinationFolder() {
  try {
    const selectedFolder = await open({
      title: 'Select Output Folder',
      directory: true,
      multiple: false
    });

    if (selectedFolder && fileStore.selectedFileIndex !== null) {
      updateFileOptions({ destinationFolder: selectedFolder as string });
    }
  } catch (error) {
    console.error('Error selecting destination folder:', error);
  }
}

// Get output filename preview
function getOutputFileName() {
  const outputName = props.selectedFile?.options?.outputName || getFileNameWithoutExtension(props.selectedFile?.name);
  const format = props.selectedFile?.options?.format || 'mp4';
  return `${outputName}.${format}`;
}

// Get filename without extension
function getFileNameWithoutExtension(filename?: string) {
  if (!filename) return 'output';
  const lastDotIndex = filename.lastIndexOf('.');
  return lastDotIndex !== -1 ? filename.substring(0, lastDotIndex) : filename;
}
</script>

<style scoped></style>
