/**
 * File helper utilities for the converter app
 */

/**
 * Extract file extension from filename
 */
export function getFileExtension(filename: string): string {
  return filename.split('.').pop()?.toLowerCase() || '';
}

/**
 * Generate output filename with new extension
 */
export function generateOutputFilename(inputFilename: string, outputFormat: string): string {
  const extension = getFileExtension(inputFilename);
  return inputFilename.replace(`.${extension}`, `.${outputFormat}`);
}

/**
 * Generate output path with destination folder and custom output name
 */
export function generateOutputPath(
  inputFilename: string,
  outputFormat: string,
  destinationFolder?: string,
  customOutputName?: string
): string {
  let outputFilename: string;

  if (customOutputName) {
    // Use custom output name with the new format extension
    outputFilename = `${customOutputName}.${outputFormat}`;
  } else {
    // Use original logic with the input filename
    outputFilename = generateOutputFilename(inputFilename, outputFormat);
  }

  return destinationFolder ? `${destinationFolder}/${outputFilename}` : outputFilename;
}

/**
 * Determine file type based on extension
 */
export function getFileTypeFromExtension(extension: string): string {
  const imageExtensions = ['jpg', 'jpeg', 'png', 'webp', 'bmp', 'tiff', 'gif', 'ico'];
  const videoExtensions = ['mp4', 'avi', 'mov', 'wmv', 'flv', 'webm', 'mkv'];

  if (imageExtensions.includes(extension)) {
    return `image/${extension === 'jpg' ? 'jpeg' : extension}`;
  } else if (videoExtensions.includes(extension)) {
    return `video/${extension}`;
  } else {
    return 'application/octet-stream';
  }
}

/**
 * Check if file is an image
 */
export function isImageFile(fileType: string): boolean {
  return fileType.startsWith('image/');
}

/**
 * Check if file is a video
 */
export function isVideoFile(fileType: string): boolean {
  return fileType.startsWith('video/');
}

/**
 * Format file size for display
 */
export function formatFileSize(bytes: number): string {
  if (bytes === 0) return '0 Bytes';

  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));

  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

/**
 * Validate file dimensions
 */
export function validateDimensions(width?: number, height?: number): boolean {
  return typeof width === 'number' && typeof height === 'number' && width > 0 && height > 0;
}

/**
 * Calculate aspect ratio
 */
export function calculateAspectRatio(width: number, height: number): number {
  return width / height;
}

/**
 * Calculate dimensions preserving aspect ratio
 */
export function calculateDimensionsWithAspectRatio(
  currentWidth: number,
  currentHeight: number,
  targetWidth?: number,
  targetHeight?: number
): { width: number; height: number } {
  const aspectRatio = calculateAspectRatio(currentWidth, currentHeight);

  if (targetWidth && !targetHeight) {
    return {
      width: targetWidth,
      height: Math.round(targetWidth / aspectRatio)
    };
  } else if (targetHeight && !targetWidth) {
    return {
      width: Math.round(targetHeight * aspectRatio),
      height: targetHeight
    };
  } else if (targetWidth && targetHeight) {
    return {
      width: targetWidth,
      height: targetHeight
    };
  } else {
    return {
      width: currentWidth,
      height: currentHeight
    };
  }
}

/**
 * Format seconds to HH:MM:SS format
 */
export function formatDuration(seconds: number): string {
  const hours = Math.floor(seconds / 3600);
  const minutes = Math.floor((seconds % 3600) / 60);
  const secs = Math.floor(seconds % 60);

  if (hours > 0) {
    return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  } else {
    return `${minutes.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  }
}

/**
 * Parse duration string (HH:MM:SS or MM:SS) to seconds
 */
export function parseDuration(duration: string): number {
  const parts = duration.split(':').map(Number);

  if (parts.length === 3) {
    // HH:MM:SS
    return parts[0] * 3600 + parts[1] * 60 + parts[2];
  } else if (parts.length === 2) {
    // MM:SS
    return parts[0] * 60 + parts[1];
  } else if (parts.length === 1) {
    // Just seconds
    return parts[0];
  }

  return 0;
}

/**
 * Calculate new duration when FPS changes (for keepFpsRatio)
 */
export function calculateDurationWithFpsRatio(originalDuration: number, originalFps: number, newFps: number): number {
  if (originalFps <= 0 || newFps <= 0) return originalDuration;

  // When FPS decreases, duration should increase to maintain the same number of frames
  // When FPS increases, duration should decrease
  return Math.round((originalDuration * originalFps) / newFps);
}
