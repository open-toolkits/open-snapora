import type { ScreenshotLocale, ScreenshotMessages } from '../types.js';

/**
 * 贴图窗口初始化数据载荷
 */
export interface PinnedImagePayload {
  /** 贴图的 PNG 图像二进制数据 */
  data: Uint8Array;
  mimeType: 'image/png';
  /** 当前界面的语言代码 */
  locale: ScreenshotLocale;
  /** 右键菜单或操作栏的国际化文案 */
  menuLabels: Pick<
    ScreenshotMessages,
    'actions' | 'copy' | 'copied' | 'save' | 'close'
  >;
}

/**
 * 贴图拖拽坐标点（屏幕物理/逻辑坐标）
 */
export interface PinnedPoint {
  x: number;
  y: number;
}

/**
 * 置顶贴图窗口与宿主交互的标准桥接接口
 */
export interface PinnedImageApi {
  /** 监听贴图数据初始化 */
  onInitialize(listener: (payload: PinnedImagePayload) => void): () => void;

  /** 监听宿主通知复制成功事件（用于播放复制反馈动画） */
  onCopied(listener: () => void): () => void;

  /** 请求宿主将贴图复制到系统剪贴板 */
  copy(): void;

  /** 请求宿主弹出文件对话框并将贴图保存到本地磁盘 */
  save(): void;

  /** 关闭当前贴图窗口 */
  close(): void;

  /** 开始窗口原生拖拽 */
  startDrag(point: PinnedPoint): void;

  /** 移动窗口位置 */
  moveDrag(point: PinnedPoint): void;

  /** 结束窗口拖拽 */
  endDrag(): void;

  /** 请求调整贴图窗口物理尺寸 */
  resize?(size: { width: number; height: number }): void;
}
