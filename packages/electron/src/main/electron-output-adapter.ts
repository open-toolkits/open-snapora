import { BrowserWindow, webContents } from 'electron';

import type { ScreenshotOptions } from '@open-snapora/shared';
import type {
  ScreenshotOutputPayload,
  ScreenshotOutputResponse,
} from '../protocol/messages.js';
import type { ScreenshotOutputContext } from './output-action-router.js';
import {
  copyPngToClipboard,
  createSuggestedName,
  savePngWithDialog,
} from './image-output.js';
import { PinnedWindowManager } from './pinned-window.js';

export interface ElectronOutputAdapterOptions {
  saveFile?: (
    data: Uint8Array,
    suggestedName: string,
    senderWebContentsId: number
  ) => Promise<string | undefined>;
  copyImage?: (data: Uint8Array) => void | Promise<void>;
  pinImage?: (
    result: ScreenshotOutputPayload['result'],
    options: ScreenshotOptions
  ) => Promise<void>;
  createSuggestedName?: () => string;
}

export interface ScreenshotOutputExecutor {
  execute(
    payload: ScreenshotOutputPayload,
    context: ScreenshotOutputContext
  ): Promise<ScreenshotOutputResponse>;
}

export class ElectronOutputAdapter implements ScreenshotOutputExecutor {
  readonly #saveFile: NonNullable<ElectronOutputAdapterOptions['saveFile']>;
  readonly #copyImage: NonNullable<ElectronOutputAdapterOptions['copyImage']>;
  readonly #pinImage: NonNullable<ElectronOutputAdapterOptions['pinImage']>;
  readonly #createSuggestedName: NonNullable<
    ElectronOutputAdapterOptions['createSuggestedName']
  >;

  constructor(options: ElectronOutputAdapterOptions = {}) {
    this.#saveFile =
      options.saveFile ??
      ((data, suggestedName, senderWebContentsId) =>
        savePngWithDialog(
          data,
          suggestedName,
          resolveOwnerWindow(senderWebContentsId)
        ));
    this.#copyImage = options.copyImage ?? copyPngToClipboard;
    let pinnedWindows: PinnedWindowManager | undefined;
    this.#pinImage =
      options.pinImage ??
      ((result, captureOptions) => {
        pinnedWindows ??= new PinnedWindowManager();
        return pinnedWindows.pin(result, captureOptions);
      });
    this.#createSuggestedName = options.createSuggestedName ?? createSuggestedName;
  }

  async execute(
    payload: ScreenshotOutputPayload,
    context: ScreenshotOutputContext
  ): Promise<ScreenshotOutputResponse> {
    if (payload.action === 'copy') {
      // 复杂逻辑注释：必须 await 等待剪贴板写入成功后再返回 completed 状态，
      // 避免截图窗口过早退出或销毁导致异步复制中断或剪贴板数据丢失。
      await this.#copyImage(payload.result.data);
      return { status: 'completed', action: 'copy' };
    }

    if (payload.action === 'pin') {
      await this.#pinImage(payload.result, context.captureOptions ?? {});
      return { status: 'completed', action: 'pin' };
    }

    const filePath = await this.#saveFile(
      payload.result.data,
      this.#createSuggestedName(),
      context.senderWebContentsId
    );
    return filePath
      ? { status: 'completed', action: 'save', filePath }
      : { status: 'cancelled' };
  }
}

function resolveOwnerWindow(senderWebContentsId: number): BrowserWindow | null {
  const sender = webContents.fromId(senderWebContentsId);
  return sender ? BrowserWindow.fromWebContents(sender) : null;
}
