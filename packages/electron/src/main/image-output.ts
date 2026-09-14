import { writeFile } from 'node:fs/promises';

import { clipboard, dialog, nativeImage } from 'electron';
import type { BrowserWindow, SaveDialogOptions } from 'electron';

export async function savePngWithDialog(
  data: Uint8Array,
  suggestedName: string,
  owner?: BrowserWindow | null
): Promise<string | undefined> {
  const options: SaveDialogOptions = {
    title: 'Save screenshot',
    defaultPath: suggestedName,
    filters: [{ name: 'PNG image', extensions: ['png'] }],
    properties: ['createDirectory', 'showOverwriteConfirmation'],
  };
  const result = owner
    ? await dialog.showSaveDialog(owner, options)
    : await dialog.showSaveDialog(options);
  if (result.canceled || !result.filePath) {
    return undefined;
  }

  await writeFile(result.filePath, data);
  return result.filePath;
}

/**
 * 将截图导出的 PNG 图像数据写入系统剪贴板。
 * 
 * 复杂逻辑说明：
 * 1. Electron 44 彻底移除了同步的 `clipboard.writeImage(image)` 接口，直接调用会抛出 TypeError。
 * 2. 官方标准替代方案是统一使用 `clipboard.write({ image }, type)`，该接口自 Electron 早期版本一直支持至今。
 * 3. 针对不同 Electron 版本的兼容策略：
 *    - 首选全版本通用的 `clipboard.write({ image })`；
 *    - 若环境较旧或存在定制 mock 不支持 `clipboard.write`，则回退调用 `clipboard.writeImage(image)`；
 *    - 若上述均不可用或抛错，提供底层的 `clipboard.writeBuffer('image/png', buffer)` 作为保底兜底。
 * 4. 异步时序保护：若底层剪贴板方法返回 Promise（或存在异步完成动作），使用 await 确保真正写入完成，
 *    避免因提前退出或关闭窗口导致剪贴板数据丢失。
 */
export async function copyPngToClipboard(data: Uint8Array): Promise<void> {
  const buffer = Buffer.from(data);
  const image = nativeImage.createFromBuffer(buffer);
  if (image.isEmpty()) {
    throw new Error('The exported PNG could not be decoded for the clipboard.');
  }

  let copied = false;
  let lastError: unknown;

  // 1. 首选策略：全版本通用的 clipboard.write({ image })
  if (typeof clipboard.write === 'function') {
    try {
      const res = (clipboard.write as (data: { image: typeof image }) => unknown)({ image });
      if (res && typeof (res as Promise<unknown>).then === 'function') {
        await res;
      }
      copied = true;
    } catch (err) {
      lastError = err;
    }
  }

  // 2. 回退策略一：兼容旧版 Electron 的 clipboard.writeImage
  if (!copied && typeof (clipboard as any).writeImage === 'function') {
    try {
      const res = (clipboard as any).writeImage(image);
      if (res && typeof (res as Promise<unknown>).then === 'function') {
        await res;
      }
      copied = true;
    } catch (err) {
      lastError = err;
    }
  }

  // 3. 回退策略二：以底层格式 buffer 写入剪贴板（image/png）
  if (!copied && typeof clipboard.writeBuffer === 'function') {
    try {
      const res: unknown = (
        clipboard.writeBuffer as (format: string, buf: Buffer) => unknown
      )('image/png', buffer);
      if (res && typeof (res as Promise<unknown>).then === 'function') {
        await res;
      }
      copied = true;
    } catch (err) {
      lastError = err;
    }
  }

  if (!copied) {
    throw (
      lastError ||
      new Error('Failed to copy image to clipboard: no supported clipboard API available in current Electron environment.')
    );
  }
}

export function createSuggestedName(): string {
  const timestamp = new Date()
    .toISOString()
    .replaceAll(':', '-')
    .replace(/\.\d{3}Z$/, '');
  return `screenshot-${timestamp}.png`;
}
