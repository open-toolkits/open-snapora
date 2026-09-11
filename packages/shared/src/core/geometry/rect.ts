/**
 * 二维坐标点
 */
export interface Point {
  x: number;
  y: number;
}

/**
 * 尺寸大小
 */
export interface Size {
  width: number;
  height: number;
}

/**
 * 矩形区域 (Point & Size)
 */
export type Rect = Point & Size;

/**
 * 根据起点和终点坐标计算规范化矩形（确保 width/height 为非负数）
 */
export function normalizeRect(start: Point, end: Point): Rect {
  return {
    x: Math.min(start.x, end.x),
    y: Math.min(start.y, end.y),
    width: Math.abs(end.x - start.x),
    height: Math.abs(end.y - start.y),
  };
}

/**
 * 将矩形裁剪到边界矩形内（防止选区溢出屏幕或画布）
 */
export function clampRect(rect: Rect, bounds: Rect): Rect {
  const left = Math.max(bounds.x, rect.x);
  const top = Math.max(bounds.y, rect.y);
  const right = Math.min(bounds.x + bounds.width, rect.x + rect.width);
  const bottom = Math.min(bounds.y + bounds.height, rect.y + rect.height);

  return {
    x: left,
    y: top,
    width: Math.max(0, right - left),
    height: Math.max(0, bottom - top),
  };
}

/**
 * 校验矩形尺寸是否达到有效阈值
 */
export function isRectValid(rect: Rect, minimumSize = 1): boolean {
  return rect.width >= minimumSize && rect.height >= minimumSize;
}

/**
 * 将视口 CSS 坐标转换到底层图像像素坐标 (Image Pixel)
 */
export function viewportPointToImagePoint(
  point: Point,
  viewportSize: Size,
  imageSize: Size
): Point {
  if (viewportSize.width <= 0 || viewportSize.height <= 0) {
    throw new RangeError('Viewport dimensions must be greater than zero.');
  }

  return {
    x: point.x * (imageSize.width / viewportSize.width),
    y: point.y * (imageSize.height / viewportSize.height),
  };
}
