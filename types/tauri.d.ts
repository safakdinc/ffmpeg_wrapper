export interface MediaInfo {
  duration?: number;
  width?: number;
  height?: number;
  video_codec?: string;
  audio_codec?: string;
  bitrate?: string;
  fps?: number;
  file_size?: number;
}

declare global {
  interface Window {
    __TAURI__?: Record<string, unknown>;
  }
}

export {};
