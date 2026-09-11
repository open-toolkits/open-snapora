import type { ScreenshotOverlayApi } from '@open-snapora/shared';

declare module '*.css' {
  const content: string;
  export default content;
}

declare global {
  interface Window {
    snaporaOverlay?: ScreenshotOverlayApi;
    __SNAPORA_MOCK__?: ScreenshotOverlayApi;
  }
}

export {};
