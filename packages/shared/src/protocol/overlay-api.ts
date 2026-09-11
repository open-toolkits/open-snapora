import type {
  ScreenshotCompletePayload,
  ScreenshotErrorPayload,
  ScreenshotFeedbackPayload,
  ScreenshotInitializePayload,
  ScreenshotOutputPayload,
  ScreenshotOutputResponse,
} from './messages.js';

/**
 * 截图 Overlay 界面与桌面宿主通信的标准桥接契约
 * 无论宿主是 Electron（经由 preload contextBridge）、Tauri（经由 tauri command/event）
 * 还是 Node+Webview（经由 websocket/rpc），Overlay UI 都通过本接口与宿主交互。
 */
export interface ScreenshotOverlayApi {
  /** 监听宿主下发的会话初始化数据（包括屏幕信息、选项配置、吸附区域等） */
  onInitialize(listener: (payload: ScreenshotInitializePayload) => void): () => void;

  /** 监听复制操作成功后的全局轻提示反馈事件 */
  onFeedback(listener: (payload: ScreenshotFeedbackPayload) => void): () => void;

  /** 向宿主确认截图会话完成，并传递最终结果 */
  confirm(payload: Omit<ScreenshotCompletePayload, 'protocolVersion'>): void;

  /** 向宿主通知用户取消了本次截图 */
  cancel(jobId: string): void;

  /** 向上层宿主上报渲染或交互过程中的错误 */
  reportError(payload: Omit<ScreenshotErrorPayload, 'protocolVersion'>): void;

  /**
   * 执行输出动作（如写入剪贴板、打开保存文件对话框落盘、创建置顶贴图窗口）
   * 由宿主原生层完成文件或剪贴板操作，并返回执行结果
   */
  output(
    payload: Omit<ScreenshotOutputPayload, 'protocolVersion'>
  ): Promise<ScreenshotOutputResponse>;

  /** 通知宿主反馈提示层已挂载并就绪 */
  feedbackReady(): void;

  /** 通知宿主 Overlay Webview 桥接已建立并准备好接收初始化数据 */
  ready(): void;

  /** 通知宿主指定任务的首帧图像已经完成解码并绘制到了画布上，宿主可安全显示窗口并赋予焦点 */
  prepared(jobId: string): void;
}
