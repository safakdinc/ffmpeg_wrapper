<template>
  <div class="w-full mx-auto bg-background-900 rounded-lg border border-white/20 shadow-sm">
    <div class="flex items-center justify-between gap-4 p-4">
      <!-- Icon/Preview Skeleton -->
      <div class="relative h-16 w-16 shrink-0 overflow-hidden rounded-md bg-background-800">
        <div class="absolute inset-0 flex items-center justify-center">
          <Icon
            :name="fileType.startsWith('image/') ? 'heroicons:photo-20-solid' : 'heroicons:video-camera-20-solid'"
            class="text-text-400"
            size="24" />
        </div>
      </div>

      <!-- File Details Skeleton -->
      <div class="flex-1 min-w-0 space-y-2 flex flex-col h-full justify-start">
        <!-- Filename -->
        <div class="relative h-fit bg-background-800 rounded overflow-hidden px-2 py-1">
          <div class="text-xs text-gray-500 dark:text-gray-400 truncate">
            {{ fileName }}
          </div>
        </div>
      </div>

      <!-- Loading Spinner -->
      <div class="flex items-center gap-2">
        <div class="animate-spin rounded-full h-6 w-6 border-2 border-gray-300 border-t-primary-500"></div>
        <span class="text-xs text-gray-500 dark:text-gray-400">Loading...</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Props {
  fileName: string;
  fileType: string;
}

defineProps<Props>();
</script>

<style scoped>
@keyframes pulse-slow {
  0%,
  100% {
    transform: translateX(-100%);
  }
  50% {
    transform: translateX(100%);
  }
}

@keyframes pulse-width {
  0% {
    width: 0%;
  }
  50% {
    width: 60%;
  }
  100% {
    width: 0%;
  }
}

.animate-pulse-slow {
  animation: pulse-slow 2s ease-in-out infinite;
}

.animate-pulse-width {
  animation: pulse-width 2s ease-in-out infinite;
}
</style>
