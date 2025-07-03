import { defineStore } from 'pinia';
import { ref } from 'vue';

export interface ImageOptions {
  format: string;
  quality: number;
  destinationFolder: string;
  resize: {
    width?: number;
    height?: number;
    maintainAspectRatio: boolean;
  };
}

export interface VideoOptions {
  format: string;
  quality: string;
  bitrate: number;
  destinationFolder: string;
  resolution: {
    width?: number;
    height?: number;
  };
  duration?: number; // in seconds
  fps?: number;
  keepFpsRatio?: boolean;
}

export const useOptionStore = defineStore('options', () => {
  // State
  const imageDefaults = ref<ImageOptions>({
    format: 'jpeg',
    quality: 80,
    destinationFolder: '',
    resize: {
      width: undefined,
      height: undefined,
      maintainAspectRatio: true
    }
  });

  const videoDefaults = ref<VideoOptions>({
    format: 'mp4',
    quality: 'medium',
    bitrate: 1000,
    destinationFolder: '',
    resolution: {
      width: undefined,
      height: undefined
    },
    duration: undefined,
    fps: 30,
    keepFpsRatio: false
  });

  // Actions
  function updateImageDefaults(options: Partial<ImageOptions>) {
    imageDefaults.value = { ...imageDefaults.value, ...options };
  }

  function updateVideoDefaults(options: Partial<VideoOptions>) {
    videoDefaults.value = { ...videoDefaults.value, ...options };
  }

  function resetImageDefaults() {
    imageDefaults.value = {
      format: 'jpeg',
      quality: 80,
      destinationFolder: '',
      resize: {
        width: undefined,
        height: undefined,
        maintainAspectRatio: true
      }
    };
  }

  function resetVideoDefaults() {
    videoDefaults.value = {
      format: 'mp4',
      quality: 'medium',
      bitrate: 1000,
      destinationFolder: '',
      resolution: {
        width: undefined,
        height: undefined
      }
    };
  }

  return {
    imageDefaults,
    videoDefaults,
    updateImageDefaults,
    updateVideoDefaults,
    resetImageDefaults,
    resetVideoDefaults
  };
});
