<template>
  <div
    v-gsap.fromTo="[
      { opacity: 0, scaleX: 0 },
      { opacity: 1, scaleX: 1, duration: 0.3, ease: 'power2.out' }
    ]"
    @click="selectFile"
    :class="[
      'relative z-40 flex w-full flex-col items-start justify-start border-2 overflow-hidden rounded-md p-4 shadow-sm cursor-pointer transition-colors duration-200',
      isSelected ? 'bg-background-900 border-2 border-primary-500/80' : 'bg-background-900 hover:bg-background-800',
      // Conversion status styling
      file.conversionStatus === 'completed'
        ? 'border-green-500/80'
        : file.conversionStatus === 'converting'
        ? 'border-yellow-500/80'
        : file.conversionStatus === 'error'
        ? 'border-red-500/80'
        : 'border-primary-500/0'
    ]">
    <div class="flex w-full items-center justify-between gap-4">
      <!-- Image Preview -->
      <div class="relative h-full aspect-square shrink-0 overflow-hidden rounded-md bg-background-800">
        <img
          v-if="getImagePreview(file)"
          :src="getImagePreview(file)"
          :alt="file.name"
          class="h-full w-full object-cover"
          @error="handleImageError" />
        <div v-else class="h-full w-full flex items-center justify-center">
          <Icon name="heroicons:photo-20-solid" class="text-neutral-400" size="24" />
        </div>
      </div>

      <!-- File Details -->
      <div class="flex-1 min-w-0">
        <p
          v-gsap.fromTo="[
            { opacity: 0, duration: 0.3, ease: 'power1.out' },
            { opacity: 1, duration: 0.3, ease: 'linear' }
          ]"
          class="truncate text-base text-neutral-700 dark:text-neutral-300">
          {{ file.name }}
        </p>

        <!-- Per-file Options - REMOVED (now in sidebar) -->

        <!-- Dimensions Display -->
        <p class="text-sm text-neutral-500 dark:text-neutral-400 mt-1" v-if="file.width && file.height">
          {{ file.width }} × {{ file.height }}
        </p>
      </div>

      <!-- File Size -->
      <p
        v-gsap.fromTo="[
          { opacity: 0, duration: 0.3, ease: 'power1.out' },
          { opacity: 1, duration: 0.3, ease: 'linear' }
        ]"
        class="w-fit shrink-0 rounded-lg px-2 py-1 text-sm text-text-100 bg-background-800 shadow-input">
        {{ formatFileSize(file.size) }}
      </p>

      <!-- Conversion Status Indicator -->
      <div class="flex items-center gap-2 h-fit">
        <!-- Converting spinner -->
        <div class="h-fit flex items-center" v-if="file.conversionStatus === 'converting'">
          <Icon name="heroicons:arrow-path-20-solid" class="text-yellow-500 animate-spin" size="20" />
        </div>
        <!-- Completed checkmark -->
        <div class="h-fit flex items-center" v-else-if="file.conversionStatus === 'completed'">
          <Icon name="heroicons:check-circle-20-solid" class="text-green-500 h-fit" size="20" />
        </div>
        <!-- Error indicator -->
        <div class="h-fit flex items-center" v-else-if="file.conversionStatus === 'error'">
          <Icon name="heroicons:exclamation-circle-20-solid" class="text-red-500" size="20" />
        </div>
      </div>

      <!-- Remove Button -->
      <button
        @click.stop="$emit('remove')"
        class="flex items-center justify-center w-8 h-8 rounded-full bg-red-500 text-white hover:bg-red-600 cursor-pointer">
        <Icon name="heroicons:trash-20-solid" size="20" />
      </button>
    </div>

    <!-- File Metadata -->
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { FileWithPath } from '@/stores/fileStore';
import { useFileStore } from '@/stores/fileStore';
import { formatFileSize } from '@/utils/fileHelpers';

interface Props {
  file: FileWithPath;
  index: number;
}

const props = defineProps<Props>();

const emit = defineEmits<{
  remove: [];
  updateOption: [options: any];
}>();

const fileStore = useFileStore();

// Check if this file is currently selected
const isSelected = computed(() => fileStore.selectedFileIndex === props.index);

// Function to select this file
function selectFile() {
  fileStore.selectFile(props.index);
}

function getImagePreview(file: FileWithPath): string {
  // For now, we'll use an icon for all files since Tauri asset protocol
  // might not work reliably for image previews
  return '';
}

function handleImageError(event: Event): void {
  console.error(`Error loading image preview for file: ${props.file.name}`, event);
}
</script>

<style scoped>
.transition-opacity {
  transition: opacity 0.3s ease;
}
</style>
