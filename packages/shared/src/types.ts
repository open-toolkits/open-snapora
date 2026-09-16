/**
 * 标注工具类型
 */
export type ScreenshotTool =
  | 'rectangle'
  | 'ellipse'
  | 'arrow'
  | 'brush'
  | 'text'
  | 'mosaic'
  | 'watermark';

/**
 * 屏幕/选区边界（全局逻辑坐标）
 */
export interface ScreenshotBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

/**
 * 支持的国际化语言代码
 */
export type ScreenshotLocale = 'en-US' | 'zh-CN' | 'ja-JP' | 'ko-KR' | 'es-ES';

/**
 * 界面文案词条
 */
export interface ScreenshotMessages {
  preparing: string;
  instruction: string;
  exporting: string;
  copied: string;
  saveCancelled: string;
  copy: string;
  cancel: string;
  save: string;
  close: string;
  pin: string;
  confirm: string;
  select: string;
  rectangle: string;
  ellipse: string;
  arrow: string;
  brush: string;
  text: string;
  textDefault: string;
  textFill: string;
  textOutline: string;
  mosaic: string;
  watermark: string;
  undo: string;
  redo: string;
  color: string;
  customColor: string;
  lineWidth: string;
  fontSize: string;
  mosaicStrength: string;
  opacity: string;
  watermarkPlaceholder: string;
  annotationCanvas: string;
  selection: string;
  actions: string;
  annotationTools: string;
  history: string;
  annotationStyle: string;
  outputActions: string;
  annotationText: string;
}

export type ScreenshotMessageOverrides = Partial<ScreenshotMessages>;

/**
 * 主题样式配置
 */
export interface ScreenshotTheme {
  mode?: 'dark' | 'light';
  accentColor?: string;
  accentForegroundColor?: string;
  maskColor?: string;
  toolbarBackground?: string;
  toolbarForeground?: string;
  toolbarBorderColor?: string;
  toolbarHoverBackground?: string;
  tooltipBackground?: string;
  tooltipForeground?: string;
  destructiveColor?: string;
  warningColor?: string;
  warningForegroundColor?: string;
  selectionHandleColor?: string;
  /** 复制成功提示的背景、文字、边框及图标颜色 */
  copyFeedbackBackground?: string;
  copyFeedbackForeground?: string;
  copyFeedbackBorderColor?: string;
  copyFeedbackIconColor?: string;
  copyFeedbackIconBackground?: string;
}

/**
 * 启动截图的调用选项
 */
export interface ScreenshotOptions {
  display?: 'cursor' | 'primary' | string;
  tools?: ScreenshotTool[];
  defaultTool?: 'select' | ScreenshotTool;
  /** 复制成功后是否显示提示，默认关闭；传 true 开启 */
  showCopyFeedback?: boolean;
  locale?: ScreenshotLocale;
  messages?: ScreenshotMessageOverrides;
  theme?: ScreenshotTheme;
}

/**
 * 统一的截图错误码
 */
export type ScreenshotErrorCode =
  | 'CAPTURE_BUSY'
  | 'INVALID_REQUEST'
  | 'RESOURCE_LIMIT_EXCEEDED'
  | 'PERMISSION_DENIED'
  | 'DISPLAY_NOT_FOUND'
  | 'CAPTURE_FAILED'
  | 'OVERLAY_LOAD_FAILED'
  | 'EXPORT_FAILED'
  | 'INVALID_RESULT'
  | 'UNSUPPORTED_PLATFORM';

/**
 * 支持的图像 MIME 规范类型
 */
export type ImageMimeType = 'image/png' | 'image/jpeg' | 'image/webp';

/**
 * 轻量图片引用契约，用于跨进程/跨层传递图片标识与规格，解耦大二进制实体
 */
export interface ImageRef {
  id: string;
  mimeType: ImageMimeType;
  width: number;
  height: number;
}

/**
 * 截图会话生命周期状态枚举
 */
export type SessionStatus =
  | 'Created'
  | 'Capturing'
  | 'OverlayReady'
  | 'Editing'
  | 'Processing'
  | 'Completed'
  | 'Cancelled'
  | 'Failed';

/**
 * 成功完成截图的图像结果数据
 */
export interface ScreenshotImageResult {
  status: 'completed';
  data: Uint8Array;
  mimeType: ImageMimeType;
  bounds: ScreenshotBounds;
  displayId: string;
}

/**
 * 输出动作的元数据
 */
export type ScreenshotOutputMetadata =
  | { action: 'copy' }
  | { action: 'save'; filePath: string }
  | { action: 'pin' };

/**
 * 截图对宿主业务返回的最终结果
 */
export type ScreenshotResult =
  | (ScreenshotImageResult & { output: ScreenshotOutputMetadata })
  | {
      status: 'cancelled';
    }
  | {
      status: 'failed';
      code: ScreenshotErrorCode;
      message: string;
    };

/**
 * 通用渲染层业务 API
 */
export interface ScreenshotRendererApi {
  capture(options?: ScreenshotOptions): Promise<ScreenshotResult>;
  cancel(): Promise<boolean>;
}
