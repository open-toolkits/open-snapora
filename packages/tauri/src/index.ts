import { invoke } from '@tauri-apps/api/core';
import type {
  ScreenshotOptions,
  ScreenshotResult,
  ScreenshotRendererApi,
} from '@open-snapora/shared';

export * from './bridge.js';
export * from '@open-snapora/shared';

interface RawRustScreenshotResult {
  status: 'completed' | 'cancelled' | 'failed';
  data?: number[];
  mimeType?: 'image/png';
  bounds?: { x: number; y: number; width: number; height: number };
  displayId?: string;
  output?: { action: 'copy' } | { action: 'save'; filePath: string } | { action: 'pin' };
  code?: any;
  message?: string;
}

/**
 * 在 Tauri 宿主应用中发起屏幕截图
 * @param options 截图选项
 */
export async function capture(
  options: ScreenshotOptions = {}
): Promise<ScreenshotResult> {
  const raw = await invoke<any>('plugin:snapora|capture', {
    options,
  });

  void invoke('plugin:snapora|log_message', {
    tag: 'Snapora:TauriJS',
    message: `capture response status: ${raw?.status}, dataLength: ${raw?.data?.length ?? 0}`
  }).catch(() => {});

  if (raw && (raw.status === 'completed' || raw.status === 'Completed') && raw.data && raw.bounds) {
    return {
      status: 'completed',
      data: new Uint8Array(raw.data),
      mimeType: raw.mimeType ?? 'image/png',
      bounds: raw.bounds,
      displayId: raw.displayId ?? 'primary',
      output: raw.output ?? { action: 'copy' },
    };
  }

  if (
    raw === 'cancelled' ||
    (raw && (raw.status === 'cancelled' || raw.status === 'Cancelled'))
  ) {
    return { status: 'cancelled' };
  }

  return {
    status: 'failed',
    code: raw?.code ?? 'CAPTURE_FAILED',
    message: raw?.message ?? (raw ? `Rust capture failed: status=${raw.status}` : 'Unknown screenshot capture error'),
  };
}

/**
 * 取消当前进行中的截图任务
 */
export async function cancel(): Promise<boolean> {
  return await invoke<boolean>('plugin:snapora|cancel_active');
}

/**
 * 默认导出的 Tauri 渲染端截图 API
 */
export const snapora: ScreenshotRendererApi = {
  capture,
  cancel,
};

export default snapora;
