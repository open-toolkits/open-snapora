import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type {
  ScreenshotOverlayApi,
  ScreenshotInitializePayload,
  ScreenshotFeedbackPayload,
  ScreenshotCompletePayload,
  ScreenshotErrorPayload,
  ScreenshotOutputPayload,
  ScreenshotOutputResponse,
  PinnedImageApi,
  PinnedImagePayload,
  PinnedPoint,
} from '@open-snapora/shared';

/**
 * 创建适用于 Tauri 运行时的 Overlay 桥接客户端
 */
export function createTauriOverlayBridge(): ScreenshotOverlayApi {
  return {
    onInitialize(listener: (payload: ScreenshotInitializePayload) => void): () => void {
      let unlisten: (() => void) | null = null;
      void listen<ScreenshotInitializePayload>('plugin:snapora:initialize', (event) => {
        listener(event.payload);
      }).then((fn) => {
        unlisten = fn;
      });

      return () => {
        unlisten?.();
        unlisten = null;
      };
    },

    onFeedback(listener: (payload: ScreenshotFeedbackPayload) => void): () => void {
      let unlisten: (() => void) | null = null;
      void listen<ScreenshotFeedbackPayload>('plugin:snapora:feedback', (event) => {
        listener(event.payload);
      }).then((fn) => {
        unlisten = fn;
      });

      return () => {
        unlisten?.();
        unlisten = null;
      };
    },

    ready(): void {
      void invoke('plugin:snapora|overlay_ready');
    },

    prepared(jobId: string): void {
      void invoke('plugin:snapora|overlay_prepared', { jobId });
    },

    feedbackReady(): void {
      void invoke('plugin:snapora|feedback_ready');
    },

    cancel(jobId: string): void {
      void invoke('plugin:snapora|cancel', { jobId });
    },

    reportError(payload: Omit<ScreenshotErrorPayload, 'protocolVersion'>): void {
      void invoke('plugin:snapora|report_error', { payload });
    },

    async output(
      payload: Omit<ScreenshotOutputPayload, 'protocolVersion'>
    ): Promise<ScreenshotOutputResponse> {
      return await invoke<ScreenshotOutputResponse>('plugin:snapora|output', {
        payload: {
          ...payload,
          // 将 Uint8Array 转换为普通数组或 ArrayBuffer 以便 serde 序列化
          result: {
            ...payload.result,
            data: Array.from(payload.result.data),
          },
        },
      });
    },

    confirm(payload: Omit<ScreenshotCompletePayload, 'protocolVersion'>): void {
      void invoke('plugin:snapora|confirm', {
        payload: {
          ...payload,
          result: {
            ...payload.result,
            data: payload.result.status === 'completed' ? Array.from(payload.result.data) : undefined,
          },
        },
      });
    },
  };
}

/**
 * 创建适用于 Tauri 运行时的置顶贴图桥接客户端
 */
export function createTauriPinnedBridge(): PinnedImageApi {
  return {
    onInitialize(listener: (payload: PinnedImagePayload) => void): () => void {
      let unlisten: (() => void) | null = null;
      void listen<PinnedImagePayload>('plugin:snapora:pinned_init', (event) => {
        listener(event.payload);
      }).then((fn) => {
        unlisten = fn;
      });
      return () => {
        unlisten?.();
        unlisten = null;
      };
    },

    onCopied(listener: () => void): () => void {
      let unlisten: (() => void) | null = null;
      void listen('plugin:snapora:pinned_copied', () => {
        listener();
      }).then((fn) => {
        unlisten = fn;
      });
      return () => {
        unlisten?.();
        unlisten = null;
      };
    },

    copy(): void {
      void invoke('plugin:snapora|pinned_copy');
    },

    save(): void {
      void invoke('plugin:snapora|pinned_save');
    },

    close(): void {
      void invoke('plugin:snapora|pinned_close');
    },

    startDrag(point: PinnedPoint): void {
      void invoke('plugin:snapora|pinned_start_drag', { point });
    },

    moveDrag(point: PinnedPoint): void {
      void invoke('plugin:snapora|pinned_move_drag', { point });
    },

    endDrag(): void {
      void invoke('plugin:snapora|pinned_end_drag');
    },
  };
}
