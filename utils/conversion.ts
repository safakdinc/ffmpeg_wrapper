import { invoke } from '@tauri-apps/api/core';
import type { FileWithPath } from '@/stores/fileStore';
import { useOptionStore } from '@/stores/optionStore';
import { generateOutputPath } from './fileHelpers';

export interface ConversionOptions {
  format: string;
  quality?: string | number;
  width?: number;
  height?: number;
  bitrate?: string;
  destinationFolder?: string;
  maintainAspectRatio?: boolean;
  // Video specific options
  duration?: number; // in seconds
  durationMode?: 'trim' | 'compress'; // Add duration mode
  fps?: number;
  keepFpsRatio?: boolean;
  disableAudio?: boolean; // Add disable audio option
}

/**
 * Convert a single image file
 */
export async function convertImage(file: FileWithPath, customOptions?: Partial<ConversionOptions>): Promise<void> {
  if (!file.path) {
    throw new Error('Cannot convert file without path');
  }

  const optionStore = useOptionStore();
  const imageDefaults = optionStore.imageDefaults;

  // Build conversion options
  const conversionOptions: ConversionOptions = {
    format: file.options?.format || customOptions?.format || imageDefaults.format,
    quality: customOptions?.quality || imageDefaults.quality,
    destinationFolder: file.options?.destinationFolder || customOptions?.destinationFolder || imageDefaults.destinationFolder,
    width: file.options?.width || customOptions?.width || imageDefaults.resize.width,
    height: file.options?.height || customOptions?.height || imageDefaults.resize.height,
    maintainAspectRatio: file.options?.maintainAspectRatio ?? customOptions?.maintainAspectRatio ?? imageDefaults.resize.maintainAspectRatio
  };

  // Generate output path
  const outputPath = generateOutputPath(file.name, conversionOptions.format, conversionOptions.destinationFolder, file.options?.outputName);

  console.log('Converting image:', file.name, 'with options:', conversionOptions);

  // Call the backend conversion command
  return invoke('start_conversion', {
    inputPath: file.path,
    outputPath: outputPath,
    format: conversionOptions.format,
    quality: conversionOptions.quality?.toString(),
    width: conversionOptions.width,
    height: conversionOptions.height
  });
}

/**
 * Convert a single video file
 */
export async function convertVideo(file: FileWithPath, customOptions?: Partial<ConversionOptions>): Promise<void> {
  if (!file.path) {
    throw new Error('Cannot convert file without path');
  }

  const optionStore = useOptionStore();
  const videoDefaults = optionStore.videoDefaults;

  // Build conversion options
  const conversionOptions: ConversionOptions = {
    format: file.options?.format || customOptions?.format || videoDefaults.format,
    quality: customOptions?.quality || videoDefaults.quality,
    bitrate: customOptions?.bitrate || videoDefaults.bitrate.toString(),
    destinationFolder: file.options?.destinationFolder || customOptions?.destinationFolder || videoDefaults.destinationFolder,
    width: file.options?.width || customOptions?.width || videoDefaults.resolution.width,
    height: file.options?.height || customOptions?.height || videoDefaults.resolution.height,
    duration: file.options?.duration || customOptions?.duration || videoDefaults.duration,
    durationMode: file.options?.durationMode || customOptions?.durationMode || 'trim', // Add duration mode
    fps: file.options?.fps || customOptions?.fps || videoDefaults.fps,
    keepFpsRatio: file.options?.keepFpsRatio ?? customOptions?.keepFpsRatio ?? videoDefaults.keepFpsRatio,
    disableAudio: file.options?.disableAudio ?? customOptions?.disableAudio ?? false // Add disable audio option
  };

  // Generate output path
  const outputPath = generateOutputPath(file.name, conversionOptions.format, conversionOptions.destinationFolder, file.options?.outputName);

  console.log('Converting video:', file.name, 'with options:', conversionOptions);

  // Call the backend conversion command
  await invoke('start_conversion', {
    inputPath: file.path,
    outputPath,
    format: conversionOptions.format,
    quality: conversionOptions.quality,
    width: conversionOptions.width,
    height: conversionOptions.height,
    duration: conversionOptions.duration,
    durationMode: conversionOptions.durationMode || 'trim', // Pass duration mode
    fps: conversionOptions.fps,
    disableAudio: conversionOptions.disableAudio // Pass disable audio option
  });
}

/**
 * Convert a file (automatically detects if it's image or video)
 */
export async function convertFile(file: FileWithPath, customOptions?: Partial<ConversionOptions>): Promise<void> {
  const isImage = file.type.startsWith('image/');

  if (isImage) {
    return convertImage(file, customOptions);
  } else {
    return convertVideo(file, customOptions);
  }
}

/**
 * Convert multiple files
 */
export async function convertMultipleFiles(
  files: FileWithPath[],
  customOptions?: Partial<ConversionOptions>
): Promise<PromiseSettledResult<void>[]> {
  const conversionPromises = files.map(file => convertFile(file, customOptions));
  return Promise.allSettled(conversionPromises);
}
