import type { Size, CapturedFrame } from '@open-snapora/shared';

export const DESKTOP_CAPTURE_TIMEOUT_MS = 2_000;

export interface DrawCapturedFrameOptions {
  signal?: AbortSignal;
  timeoutMs?: number;
}

/**
 * 帧加载器函数签名：支持宿主自定义帧解码逻辑（如 Electron 桌面流或 Tauri 内存映射）
 */
export type FrameLoader = (
  canvas: HTMLCanvasElement,
  frame: CapturedFrame,
  options?: DrawCapturedFrameOptions
) => Promise<Size>;

/**
 * 默认标准 Web 图像帧绘制实现：解析 Base64 / Blob DataURL 并绘制到 Canvas
 */
export async function drawCapturedFrame(
  canvas: HTMLCanvasElement,
  frame: CapturedFrame,
  options: DrawCapturedFrameOptions = {}
): Promise<Size> {
  if (frame.kind === 'desktop-source') {
    throw new Error(
      '[open-snapora] Desktop source frame requires a host-specific frame loader (e.g. Electron media stream).'
    );
  }

  const image = await loadImage(frame.dataUrl, options.signal);
  canvas.width = frame.pixelSize.width;
  canvas.height = frame.pixelSize.height;
  const context = requireCanvasContext(canvas);
  context.drawImage(image, 0, 0, canvas.width, canvas.height);
  return frame.pixelSize;
}

function requireCanvasContext(canvas: HTMLCanvasElement): CanvasRenderingContext2D {
  const context = canvas.getContext('2d');
  if (!context) {
    throw new Error('[open-snapora] Could not create 2D canvas rendering context.');
  }
  return context;
}

function loadImage(dataUrl: string, signal?: AbortSignal): Promise<HTMLImageElement> {
  return new Promise((resolve, reject) => {
    const image = new Image();
    const cleanup = () => {
      image.onload = null;
      image.onerror = null;
      signal?.removeEventListener('abort', handleAbort);
    };
    const handleAbort = () => {
      cleanup();
      reject(new DOMException('Image load was aborted.', 'AbortError'));
    };
    if (signal?.aborted) {
      handleAbort();
      return;
    }
    signal?.addEventListener('abort', handleAbort, { once: true });
    image.onload = () => {
      cleanup();
      resolve(image);
    };
    image.onerror = () => {
      cleanup();
      reject(new Error('[open-snapora] Failed to load screenshot image frame into DOM.'));
    };
    image.src = dataUrl;
  });
}
