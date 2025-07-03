<template>
  <div class="w-[350px] h-full overflow-auto p-4 bg-background-900 space-y-4 rounded-lg">
    <h2 class="text-lg font-bold text-neutral-700 dark:text-neutral-300 mb-4">File Options</h2>

    <!-- No file selected state -->
    <div v-if="!selectedFile" class="text-center py-8">
      <Icon name="heroicons:document-20-solid" size="48" class="mx-auto text-neutral-400 mb-4" />
      <p class="text-neutral-500 dark:text-neutral-400">Select a file to edit its conversion options</p>
    </div>

    <!-- Selected file options -->
    <div v-else class="space-y-4">
      <!-- File Info -->
      <div class="bg-background-800 p-3 rounded-lg">
        <div class="flex items-center gap-3">
          <Icon
            :name="selectedFile.type?.startsWith('image/') ? 'heroicons:photo-20-solid' : 'heroicons:video-camera-20-solid'"
            size="24"
            class="text-neutral-400" />
          <div class="flex-1 min-w-0">
            <p class="text-sm font-medium text-neutral-700 dark:text-neutral-300 truncate">
              {{ selectedFile.name }}
            </p>
            <p class="text-xs text-neutral-500 dark:text-neutral-400">
              {{ formatFileSize(selectedFile.size) }}
              <span v-if="selectedFile.width && selectedFile.height"> • {{ selectedFile.width }} × {{ selectedFile.height }} </span>
            </p>
          </div>
        </div>
      </div>

      <!-- Image Options -->
      <OptionsImage v-if="selectedFile.type?.startsWith('image/')" :selected-file="selectedFile" />

      <!-- Video Options -->
      <OptionsVideo v-else-if="selectedFile.type?.startsWith('video/')" :selected-file="selectedFile" />

      <!-- Convert Button -->
      <div class="pt-4 border-t border-gray-300 dark:border-gray-600">
        <UButton
          @click="convertSelectedFile"
          :disabled="converting"
          block
          :loading="converting"
          :icon="converting ? 'i-heroicons-arrow-path-20-solid' : 'i-heroicons-arrow-path-20-solid'"
          color="primary">
          {{ converting ? 'Converting...' : 'Convert File' }}
        </UButton>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useFileStore } from '@/stores/fileStore';
import { formatFileSize } from '@/utils/fileHelpers';
import { convertImage, convertVideo } from '@/utils/conversion';

const fileStore = useFileStore();
const converting = ref(false);

// Get the selected file from the store
const selectedFile = computed(() => fileStore.getSelectedFile());

// Convert the selected file
async function convertSelectedFile() {
  if (!selectedFile.value || converting.value) return;

  converting.value = true;
  const fileIndex = fileStore.selectedFileIndex;

  if (fileIndex === null) {
    converting.value = false;
    return;
  }

  try {
    // Set converting status
    fileStore.setConversionStatus(fileIndex, 'converting');

    if (selectedFile.value.type?.startsWith('image/')) {
      await convertImage(selectedFile.value);
    } else if (selectedFile.value.type?.startsWith('video/')) {
      await convertVideo(selectedFile.value);
    }

    // Set completed status
    fileStore.setConversionStatus(fileIndex, 'completed');
    console.log('File conversion completed successfully');
  } catch (error) {
    // Set error status
    fileStore.setConversionStatus(fileIndex, 'error', error instanceof Error ? error.message : 'Unknown error');
    console.error('File conversion failed:', error);
  } finally {
    converting.value = false;
  }
}
</script>

<style scoped>
/* Add any additional styles if needed */
</style>
