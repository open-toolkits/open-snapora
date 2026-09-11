import type {
  ScreenshotBounds,
  ScreenshotErrorCode,
  ScreenshotImageResult,
  ScreenshotOptions,
  ScreenshotResult,
} from '../types.js';

/**
 * 协议版本号常量
 */
export const SCREENSHOT_PROTOCOL_VERSION = 2 as const;

export interface ScreenshotReadyPayload {
  protocolVersion: typeof SCREENSHOT_PROTOCOL_VERSION;
}

export interface ScreenshotPreparedPayload extends ScreenshotReadyPayload {
  jobId: string;
}

/**
 * 屏幕显示器信息元数据
 */
export interface CaptureDisplay {
  /** 宿主内的稳定显示器标识 */
  id: string;
  /** 显示器在多屏空间中的逻辑边界（允许负 x/y） */
  bounds: ScreenshotBounds;
  /** 显示器缩放比例（DPI scale factor） */
  scaleFactor: number;
}

/**
 * 通用图片帧（Base64 DataURL 或图片资源）
 */
export interface CapturedImageFrame {
  /** 缺省时兼容旧 1.x 自定义适配器 */
  kind?: 'image';
  display: CaptureDisplay;
  dataUrl: string;
  pixelSize: {
    width: number;
    height: number;
  };
}

/**
 * Electron 专属的桌面音视频流源帧（通过 desktopCapturer 配合 WebRTC stream 零拷贝解码）
 */
export interface CapturedDesktopSourceFrame {
  kind: 'desktop-source';
  display: CaptureDisplay;
  sourceId: string;
  pixelSize: {
    width: number;
    height: number;
  };
}

/**
 * 捕获帧联合类型
 */
export type CapturedFrame = CapturedImageFrame | CapturedDesktopSourceFrame;

/**
 * 宿主屏幕捕获适配器接口
 */
export interface ScreenCaptureAdapter {
  /** 可选的后台预热，用于把昂贵的来源枚举移出用户点击路径 */
  prepare?(): Promise<void>;
  /**
   * 可选的同步目标解析，用于让主进程在屏幕采集期间并行加载隐藏 Overlay
   * 返回值会作为锁定目标传给紧随其后的 capture()，避免鼠标跨屏造成截图与窗口错位
   */
  resolveTargetDisplay?(options: ScreenshotOptions): CaptureDisplay;
  capture(
    options: ScreenshotOptions,
    targetDisplay?: CaptureDisplay
  ): Promise<CapturedFrame[]>;
  /** Renderer 无法读取 desktop-source 时，可返回图片帧完成本次截图 */
  captureFallback?(
    options: ScreenshotOptions,
    targetDisplay: CaptureDisplay
  ): Promise<CapturedFrame[]>;
}

/**
 * 初始化会话数据包
 */
export interface ScreenshotInitializePayload {
  protocolVersion: typeof SCREENSHOT_PROTOCOL_VERSION;
  jobId: string;
  options: ScreenshotOptions;
  frames: CapturedFrame[];
  /** 可单击吸附的可见窗口区域，使用全局 Screen DIP 坐标 */
  windowSnapRegions?: ScreenshotBounds[];
}

/**
 * 复制成功的全局轻提示数据包
 */
export interface ScreenshotFeedbackPayload {
  kind: 'copy';
  durationMs: number;
  options: ScreenshotOptions;
}

/**
 * 完成截图数据包
 */
export interface ScreenshotCompletePayload {
  protocolVersion: typeof SCREENSHOT_PROTOCOL_VERSION;
  jobId: string;
  result: ScreenshotResult;
}

/**
 * 取消截图数据包
 */
export interface ScreenshotCancelPayload {
  protocolVersion: typeof SCREENSHOT_PROTOCOL_VERSION;
  jobId: string;
}

/**
 * 错误上报数据包
 */
export interface ScreenshotErrorPayload {
  protocolVersion: typeof SCREENSHOT_PROTOCOL_VERSION;
  jobId: string;
  code: ScreenshotErrorCode;
  message: string;
  fallback?: 'capture-image';
}

/**
 * 输出动作类型
 */
export type ScreenshotOutputAction = 'save' | 'copy' | 'pin';

/**
 * 请求输出动作载荷
 */
export interface ScreenshotOutputPayload {
  protocolVersion: typeof SCREENSHOT_PROTOCOL_VERSION;
  jobId: string;
  action: ScreenshotOutputAction;
  result: ScreenshotImageResult;
}

/**
 * 输出动作响应结果
 */
export type ScreenshotOutputResponse =
  | { status: 'completed'; action: 'copy' }
  | { status: 'completed'; action: 'save'; filePath: string }
  | { status: 'completed'; action: 'pin' }
  | { status: 'cancelled' }
  | { status: 'failed'; code: ScreenshotErrorCode; message: string };
