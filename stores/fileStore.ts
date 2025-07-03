import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface FileFormatOptions {
  format?: string;
  outputName?: string; // Custom output filename (without extension)
  width?: number;
  height?: number;
  originalWidth?: number; // Store original dimensions for aspect ratio calculations
  originalHeight?: number;
  originalAspectRatio?: number; // Store original aspect ratio (width / height)
  targetWidth?: number; // Target output dimensions
  targetHeight?: number;
  maintainAspectRatio?: boolean;
  destinationFolder?: string;
  // Video specific options
  duration?: number;
  durationMode?: 'trim' | 'compress'; // Add duration mode
  fps?: number;
  keepFpsRatio?: boolean;
  disableAudio?: boolean; // Add disable audio option
}

export interface FileWithPath {
  name: string;
  size: number;
  type: string;
  lastModified: number;
  path: string;
  width?: number;
  height?: number;
  options?: FileFormatOptions;
  // Conversion status tracking
  conversionStatus?: 'idle' | 'converting' | 'completed' | 'error';
  conversionError?: string;
}

export const useFileStore = defineStore('fileStore', () => {
  const selectedFiles = ref<FileWithPath[]>([]);
  const selectedFileIndex = ref<number | null>(null);

  function addFiles(files: FileWithPath[]) {
    files.forEach(newFile => {
      const isVideo = newFile.type?.startsWith('video/');
      if (!newFile.options) newFile.options = {};

      // Patch missing fields
      if (newFile.width && !newFile.options.originalWidth) newFile.options.originalWidth = newFile.width;
      if (newFile.height && !newFile.options.originalHeight) newFile.options.originalHeight = newFile.height;
      if (newFile.options.originalWidth && newFile.options.originalHeight && !newFile.options.originalAspectRatio) {
        newFile.options.originalAspectRatio = newFile.options.originalWidth / newFile.options.originalHeight;
      }
      if (newFile.width && !newFile.options.targetWidth) newFile.options.targetWidth = newFile.width;
      if (newFile.height && !newFile.options.targetHeight) newFile.options.targetHeight = newFile.height;
      if (newFile.options.maintainAspectRatio === undefined) newFile.options.maintainAspectRatio = true;
      if (newFile.options.format === undefined) newFile.options.format = isVideo ? 'mp4' : 'jpeg';

      // Set default output name to original filename without extension
      if (newFile.options.outputName === undefined) {
        const fileName = newFile.name;
        const lastDotIndex = fileName.lastIndexOf('.');
        newFile.options.outputName = lastDotIndex !== -1 ? fileName.substring(0, lastDotIndex) : fileName;
      }

      // Set default destinationFolder to file's original folder if not set
      if (newFile.options.destinationFolder === undefined || newFile.options.destinationFolder === '') {
        // Extract folder from file path
        const lastSlash = newFile.path.lastIndexOf('/') !== -1 ? newFile.path.lastIndexOf('/') : newFile.path.lastIndexOf('\\');
        newFile.options.destinationFolder = lastSlash !== -1 ? newFile.path.slice(0, lastSlash) : '';
      }

      if (isVideo) {
        if (newFile.options.duration === undefined) newFile.options.duration = undefined;
        if (newFile.options.fps === undefined) newFile.options.fps = 30;
        if (newFile.options.keepFpsRatio === undefined) newFile.options.keepFpsRatio = false;
        if (newFile.options.disableAudio === undefined) newFile.options.disableAudio = false;
      }
      const idx = selectedFiles.value.findIndex(f => f.path === newFile.path);
      if (idx !== -1) {
        selectedFiles.value[idx] = { ...selectedFiles.value[idx], ...newFile };
      } else {
        selectedFiles.value.push({ ...newFile });
      }
    });

    // Auto-select first file if none selected
    if (selectedFileIndex.value === null && selectedFiles.value.length > 0) {
      selectedFileIndex.value = 0;
    }
  }

  function selectFile(index: number) {
    if (index >= 0 && index < selectedFiles.value.length) {
      selectedFileIndex.value = index;
    }
  }

  function getSelectedFile(): FileWithPath | null {
    if (selectedFileIndex.value !== null && selectedFiles.value[selectedFileIndex.value]) {
      return selectedFiles.value[selectedFileIndex.value];
    }
    return null;
  }

  function updateFileOptions(index: number, options: Partial<FileFormatOptions>) {
    const file = selectedFiles.value[index];
    if (!file) return;

    // Aspect ratio logic for targetWidth/targetHeight
    if (
      options.targetWidth !== undefined &&
      file.options?.originalWidth &&
      file.options?.originalHeight &&
      (file.options?.maintainAspectRatio ?? true)
    ) {
      const aspect = file.options.originalHeight / file.options.originalWidth;
      options.targetHeight = Math.round(options.targetWidth * aspect);
    } else if (
      options.targetHeight !== undefined &&
      file.options?.originalWidth &&
      file.options?.originalHeight &&
      (file.options?.maintainAspectRatio ?? true)
    ) {
      const aspect = file.options.originalWidth / file.options.originalHeight;
      options.targetWidth = Math.round(options.targetHeight * aspect);
    }

    // Always sync width/height to targetWidth/targetHeight for backend
    if (options.targetWidth !== undefined) {
      options.width = options.targetWidth;
    }
    if (options.targetHeight !== undefined) {
      options.height = options.targetHeight;
    }

    // FPS ratio logic for videos
    if (options.fps !== undefined && file.options?.keepFpsRatio && file.options?.fps && file.options?.duration) {
      const originalFps = file.options.fps;
      const originalDuration = file.options.duration;
      const newFps = options.fps;

      // Calculate new duration based on FPS change
      options.duration = Math.round((originalDuration * originalFps) / newFps);
    }

    file.options = { ...file.options, ...options };
  }

  function removeFile(index: number) {
    selectedFiles.value.splice(index, 1);

    // Adjust selected index after removal
    if (selectedFileIndex.value === index) {
      // If removed file was selected, select next one or previous one
      if (selectedFiles.value.length === 0) {
        selectedFileIndex.value = null;
      } else if (index >= selectedFiles.value.length) {
        selectedFileIndex.value = selectedFiles.value.length - 1;
      }
      // else keep the same index (which now points to the next file)
    } else if (selectedFileIndex.value !== null && selectedFileIndex.value > index) {
      // If removed file was before selected file, adjust index
      selectedFileIndex.value--;
    }
  }

  function clearFiles() {
    selectedFiles.value = [];
    selectedFileIndex.value = null;
  }

  function setConversionStatus(index: number, status: 'idle' | 'converting' | 'completed' | 'error', error?: string) {
    const file = selectedFiles.value[index];
    if (file) {
      file.conversionStatus = status;
      if (error) {
        file.conversionError = error;
      } else {
        file.conversionError = undefined;
      }
    }
  }

  function getConversionStatus(index: number): 'idle' | 'converting' | 'completed' | 'error' {
    const file = selectedFiles.value[index];
    return file?.conversionStatus || 'idle';
  }

  return {
    selectedFiles,
    selectedFileIndex,
    addFiles,
    removeFile,
    clearFiles,
    updateFileOptions,
    selectFile,
    getSelectedFile,
    setConversionStatus,
    getConversionStatus
  };
});
