import type {
  ScreenshotInitializePayload,
  ScreenshotOverlayApi,
  ScreenshotFeedbackPayload,
  ScreenshotCompletePayload,
  ScreenshotErrorPayload,
  ScreenshotOutputPayload,
  ScreenshotOutputResponse,
} from '@open-snapora/shared';

/**
 * 生成一张用于浏览器独立调试的高清测试底图 (DataURL)
 */
export function createMockScreenDataUrl(
  width = 1920,
  height = 1080
): string {
  const canvas = document.createElement('canvas');
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');
  if (!ctx) return '';

  // 绘制背景渐变
  const gradient = ctx.createLinearGradient(0, 0, width, height);
  gradient.addColorStop(0, '#1e1e2e');
  gradient.addColorStop(0.5, '#282a36');
  gradient.addColorStop(1, '#181825');
  ctx.fillStyle = gradient;
  ctx.fillRect(0, 0, width, height);

  // 绘制网格背景
  ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
  ctx.lineWidth = 1;
  const step = 40;
  for (let x = 0; x < width; x += step) {
    ctx.beginPath();
    ctx.moveTo(x, 0);
    ctx.lineTo(x, height);
    ctx.stroke();
  }
  for (let y = 0; y < height; y += step) {
    ctx.beginPath();
    ctx.moveTo(0, y);
    ctx.lineTo(width, y);
    ctx.stroke();
  }

  // 绘制中心测试卡与说明文字
  ctx.fillStyle = '#ffffff';
  ctx.font = 'bold 36px system-ui, sans-serif';
  ctx.textAlign = 'center';
  ctx.fillText('open-snapora: 跨宿主截图与标注独立调试画布', width / 2, height / 2 - 40);

  ctx.fillStyle = '#89b4fa';
  ctx.font = '20px system-ui, sans-serif';
  ctx.fillText(
    '当前运行于纯浏览器环境 (Standalone Browser Mock)。支持矩形、椭圆、画笔、文字、马赛克、撤销/重做与导出！',
    width / 2,
    height / 2 + 10
  );

  ctx.fillStyle = '#a6adc8';
  ctx.font = '16px monospace';
  ctx.fillText(`Canvas Resolution: ${width} × ${height} | Ratio: 100%`, width / 2, height / 2 + 50);

  // 绘制一些色块便于测试马赛克与选区
  const colors = ['#f38ba8', '#fab387', '#f9e2af', '#a6e3a1', '#89dceb', '#b4befe', '#cba6f7'];
  colors.forEach((col, idx) => {
    ctx.fillStyle = col;
    ctx.fillRect(width / 2 - 210 + idx * 60, height / 2 + 90, 50, 50);
  });

  return canvas.toDataURL('image/png');
}

/**
 * 创建用于脱机浏览器调试的 Mock ScreenshotOverlayApi 桥接对象
 */
export function createMockOverlayBridge(): ScreenshotOverlayApi {
  let initListener: ((payload: ScreenshotInitializePayload) => void) | null = null;
  let feedbackListener: ((payload: ScreenshotFeedbackPayload) => void) | null = null;

  return {
    onInitialize(listener) {
      initListener = listener;
      return () => {
        initListener = null;
      };
    },
    onFeedback(listener) {
      feedbackListener = listener;
      return () => {
        feedbackListener = null;
      };
    },
    ready() {
      console.log('[MockBridge] Overlay ready, emitting mock initialization payload...');
      // 延迟微任务下发初始化，确保监听器完成挂载
      setTimeout(() => {
        if (initListener) {
          const mockDataUrl = createMockScreenDataUrl();
          initListener({
            protocolVersion: 2,
            jobId: `mock-job-${Date.now()}`,
            options: {
              locale: 'zh-CN',
              showCopyFeedback: true,
              defaultTool: 'select',
            },
            frames: [
              {
                kind: 'image',
                display: {
                  id: 'mock-display-0',
                  bounds: { x: 0, y: 0, width: 1920, height: 1080 },
                  scaleFactor: 1,
                },
                dataUrl: mockDataUrl,
                pixelSize: { width: 1920, height: 1080 },
              },
            ],
            windowSnapRegions: [
              { x: 300, y: 200, width: 600, height: 400 },
              { x: 1000, y: 250, width: 700, height: 500 },
            ],
          });
        }
      }, 50);
    },
    prepared(jobId) {
      console.log(`[MockBridge] Prepared frame for jobId: ${jobId}`);
    },
    feedbackReady() {
      console.log('[MockBridge] Feedback toast rendered');
    },
    cancel(jobId) {
      console.warn(`[MockBridge] User cancelled screenshot session: ${jobId}`);
      alert('已取消截图！');
    },
    reportError(payload: Omit<ScreenshotErrorPayload, 'protocolVersion'>) {
      console.error('[MockBridge] Overlay reported error:', payload);
    },
    async output(payload: Omit<ScreenshotOutputPayload, 'protocolVersion'>): Promise<ScreenshotOutputResponse> {
      console.log('[MockBridge] Output action received:', payload.action, payload.result);

      if (payload.action === 'save') {
        // 在浏览器中模拟保存：直接触发下载 PNG 文件
        const blob = new Blob([payload.result.data.buffer as ArrayBuffer], { type: 'image/png' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `open-snapora-capture-${Date.now()}.png`;
        a.click();
        URL.revokeObjectURL(url);
        return { status: 'completed', action: 'save', filePath: a.download };
      }

      if (payload.action === 'copy') {
        try {
          if (navigator.clipboard && typeof ClipboardItem !== 'undefined') {
            const blob = new Blob([payload.result.data.buffer as ArrayBuffer], { type: 'image/png' });
            await navigator.clipboard.write([new ClipboardItem({ 'image/png': blob })]);
            console.log('[MockBridge] Image written to system clipboard successfully.');
          }
        } catch (err) {
          console.warn('[MockBridge] Navigator clipboard write error:', err);
        }
        return { status: 'completed', action: 'copy' };
      }

      return { status: 'completed', action: 'pin' };
    },
    confirm(payload: Omit<ScreenshotCompletePayload, 'protocolVersion'>) {
      console.log('[MockBridge] Screenshot session completed successfully:', payload.result);
      if (feedbackListener) {
        feedbackListener({
          kind: 'copy',
          durationMs: 1500,
          options: { locale: 'zh-CN' },
        });
      }
    },
  };
}
