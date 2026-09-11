import type { CapturedFrame } from './messages.js';

/**
 * 截图资源限制参数
 */
export interface ScreenshotResourceLimits {
  /** 最大捕获像素量（宽 * 高） */
  maxCapturePixels: number;
  /** 最大 DataURL 字符长度（字节预算） */
  maxCaptureDataUrlBytes: number;
  /** 最大输出图片字节大小 */
  maxOutputBytes: number;
}

export type ScreenshotResourceLimitOptions = Partial<ScreenshotResourceLimits>;

/**
 * 默认资源限制：64M 像素，192MB DataURL，64MB 输出文件
 */
export const DEFAULT_SCREENSHOT_RESOURCE_LIMITS: Readonly<ScreenshotResourceLimits> =
  Object.freeze({
    maxCapturePixels: 64 * 1024 * 1024,
    maxCaptureDataUrlBytes: 192 * 1024 * 1024,
    maxOutputBytes: 64 * 1024 * 1024,
  });

/**
 * 硬限制阈值
 */
export const HARD_SCREENSHOT_RESOURCE_LIMITS: Readonly<ScreenshotResourceLimits> =
  Object.freeze({
    maxCapturePixels: 128 * 1024 * 1024,
    maxCaptureDataUrlBytes: 256 * 1024 * 1024,
    maxOutputBytes: 256 * 1024 * 1024,
  });

/**
 * 解析合并资源限制配置
 */
export function resolveScreenshotResourceLimits(
  options: ScreenshotResourceLimitOptions = {}
): ScreenshotResourceLimits {
  return {
    maxCapturePixels: resolveLimit('maxCapturePixels', options.maxCapturePixels),
    maxCaptureDataUrlBytes: resolveLimit(
      'maxCaptureDataUrlBytes',
      options.maxCaptureDataUrlBytes
    ),
    maxOutputBytes: resolveLimit('maxOutputBytes', options.maxOutputBytes),
  };
}

/**
 * 检查捕获帧是否超出资源限制，若超限则返回原因描述
 */
export function findCapturedFrameLimitViolation(
  frame: CapturedFrame,
  limits: ScreenshotResourceLimits
): string | undefined {
  const { width, height } = frame.pixelSize;
  if (
    !Number.isFinite(width) ||
    !Number.isFinite(height) ||
    width <= 0 ||
    height <= 0 ||
    width * height > limits.maxCapturePixels
  ) {
    return `Captured frame exceeds the ${limits.maxCapturePixels} pixel limit.`;
  }
  if (
    frame.kind !== 'desktop-source' &&
    frame.dataUrl &&
    frame.dataUrl.length > limits.maxCaptureDataUrlBytes
  ) {
    return `Captured frame exceeds the ${limits.maxCaptureDataUrlBytes} byte Data URL limit.`;
  }
  return undefined;
}

function resolveLimit(
  key: keyof ScreenshotResourceLimits,
  configuredValue: number | undefined
): number {
  const value = configuredValue ?? DEFAULT_SCREENSHOT_RESOURCE_LIMITS[key];
  if (
    !Number.isSafeInteger(value) ||
    value <= 0 ||
    value > HARD_SCREENSHOT_RESOURCE_LIMITS[key]
  ) {
    throw new TypeError(
      `[open-snapora] ${key} must be a positive safe integer no greater than ${HARD_SCREENSHOT_RESOURCE_LIMITS[key]}.`
    );
  }
  return value;
}
