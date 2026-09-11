import type { Point, Rect } from '../geometry/rect.js';

/**
 * 标注图形基类属性
 */
interface AnnotationElementBase {
  id: string;
  zIndex: number;
  createdAt: number;
  color: string;
}

/**
 * 矩形标注
 */
export interface RectangleElement extends AnnotationElementBase {
  type: 'rectangle';
  bounds: Rect;
  lineWidth: number;
}

/**
 * 椭圆标注
 */
export interface EllipseElement extends AnnotationElementBase {
  type: 'ellipse';
  bounds: Rect;
  lineWidth: number;
}

/**
 * 箭头标注
 */
export interface ArrowElement extends AnnotationElementBase {
  type: 'arrow';
  start: Point;
  end: Point;
  lineWidth: number;
}

/**
 * 画笔自由涂鸦
 */
export interface BrushElement extends AnnotationElementBase {
  type: 'brush';
  points: Point[];
  lineWidth: number;
}

/**
 * 文字布局尺寸度量
 */
export interface TextLayoutMetrics {
  width: number;
  ascent: number;
  descent: number;
}

/**
 * 文字样式风格：普通文字、色块填充、阴影；outline 保持向下兼容
 */
export type TextStyle = 'default' | 'fill' | 'shadow' | 'outline';

/**
 * 文本标注
 */
export interface TextElement extends AnnotationElementBase {
  type: 'text';
  position: Point;
  value: string;
  fontSize: number;
  metrics: TextLayoutMetrics;
  /** 缺省时兼容旧文档并按普通文字渲染 */
  textStyle?: TextStyle;
  /** 填充预设复用输入框内容区域，确保编辑态与 Canvas 背景边界一致 */
  fillBounds?: Rect;
  /** 记录输入态容器的真实盒模型边界（Image Pixel），确保选中态、拖拽态与输入态 100% 像素级对齐 */
  inputBounds?: Rect;
}

/**
 * 马赛克模糊区域
 */
export interface MosaicElement extends AnnotationElementBase {
  type: 'mosaic';
  bounds: Rect;
  /** 马赛克块边长，使用 Image Pixel；缺省时回退到 8 */
  blockSize?: number;
}

/**
 * 标注元素联合类型
 */
export type AnnotationElement =
  | RectangleElement
  | EllipseElement
  | ArrowElement
  | BrushElement
  | TextElement
  | MosaicElement;

/**
 * 完整的截图标注文档模型（包含选区及所有标注元素）
 */
export interface ScreenshotDocument {
  selection: Rect;
  elements: AnnotationElement[];
}
