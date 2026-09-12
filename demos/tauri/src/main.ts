import { capture, prewarm } from '@open-snapora/tauri';
import { invoke } from '@tauri-apps/api/core';
import { createRendererLogger } from 'desklog/renderer';

const logger = createRendererLogger({
  defaultModule: 'TauriDemo',
  enableConsole: true,
  send: (payload) => {
    // 调用插件将前端 desklog 日志汇入项目 logs/snapora.log 文件
    void invoke('plugin:snapora|log_message', {
      tag: `Desklog:${payload.module || 'app'}`,
      message: `[${payload.level}] ${payload.message} ${payload.data ? JSON.stringify(payload.data) : ''}`
    }).catch(() => {});
  },
});

logger.info('Tauri Demo 页面初始化就绪');

// 主界面运行起来后，在后台静默预热透明截图遮罩窗口，确保使用时直接秒级展开
setTimeout(() => {
  void prewarm()
    .then(() => {
      logger.info('截图遮罩窗口后台预热就绪，后续截屏将实现毫秒级秒开');
    })
    .catch((err) => {
      logger.warn('截图遮罩窗口预热异常', err);
    });
}, 500);

const captureBtn = document.getElementById('captureBtn') as HTMLButtonElement;
const outputLog = document.getElementById('outputLog') as HTMLPreElement;

// 异步获取并显示系统 AppData 日志路径
void invoke<string>('plugin:snapora|get_log_path').then((path) => {
  logger.info('系统日志文件已就绪', { path });
  outputLog.textContent = `// 操作系统日志文件: ${path}\n// 就绪状态：点击下方按钮或按快捷键开始截屏...`;
}).catch(() => {});


// 快捷键设置相关 DOM
const hotkeyDisplayView = document.getElementById('hotkeyDisplayView') as HTMLDivElement;
const hotkeyEditView = document.getElementById('hotkeyEditView') as HTMLDivElement;
const currentHotkeyDisplay = document.getElementById('currentHotkeyDisplay') as HTMLDivElement;
const editHotkeyBtn = document.getElementById('editHotkeyBtn') as HTMLButtonElement;
const recordingText = document.getElementById('recordingText') as HTMLSpanElement;
const saveHotkeyBtn = document.getElementById('saveHotkeyBtn') as HTMLButtonElement;
const cancelHotkeyBtn = document.getElementById('cancelHotkeyBtn') as HTMLButtonElement;

interface HotkeyConfig {
  ctrl: boolean;
  alt: boolean;
  shift: boolean;
  meta: boolean;
  code: string;
  keyName: string;
}

const DEFAULT_HOTKEY: HotkeyConfig = {
  ctrl: true,
  alt: false,
  shift: true,
  meta: false,
  code: 'KeyA',
  keyName: 'A',
};

function loadSavedHotkey(): HotkeyConfig {
  try {
    const raw = localStorage.getItem('snapora_custom_hotkey');
    if (raw) {
      return JSON.parse(raw);
    }
  } catch {}
  return DEFAULT_HOTKEY;
}

let activeHotkey: HotkeyConfig = loadSavedHotkey();
let tempRecordedHotkey: HotkeyConfig | null = null;
let isRecording = false;

function renderHotkeyDisplay(config: HotkeyConfig) {
  const parts: string[] = [];
  if (config.ctrl) parts.push('<kbd>Ctrl</kbd>');
  if (config.meta) parts.push('<kbd>Cmd</kbd>');
  if (config.alt) parts.push('<kbd>Alt</kbd>');
  if (config.shift) parts.push('<kbd>Shift</kbd>');
  parts.push(`<kbd>${config.keyName.toUpperCase()}</kbd>`);
  currentHotkeyDisplay.innerHTML = parts.join(' + ');
}

renderHotkeyDisplay(activeHotkey);

// 进入修改模式
editHotkeyBtn.addEventListener('click', () => {
  isRecording = true;
  tempRecordedHotkey = null;
  recordingText.textContent = '请在键盘上按下组合键...';
  hotkeyDisplayView.style.display = 'none';
  hotkeyEditView.style.display = 'flex';
});

// 取消修改
cancelHotkeyBtn.addEventListener('click', () => {
  isRecording = false;
  tempRecordedHotkey = null;
  hotkeyEditView.style.display = 'none';
  hotkeyDisplayView.style.display = 'flex';
});

// 保存新快捷键
saveHotkeyBtn.addEventListener('click', () => {
  if (tempRecordedHotkey) {
    activeHotkey = tempRecordedHotkey;
    localStorage.setItem('snapora_custom_hotkey', JSON.stringify(activeHotkey));
    renderHotkeyDisplay(activeHotkey);
    outputLog.textContent = `✅ 快捷键已更新为: ${formatHotkeyName(activeHotkey)}`;
  }
  isRecording = false;
  tempRecordedHotkey = null;
  hotkeyEditView.style.display = 'none';
  hotkeyDisplayView.style.display = 'flex';
});

function formatHotkeyName(config: HotkeyConfig): string {
  const names: string[] = [];
  if (config.ctrl) names.push('Ctrl');
  if (config.meta) names.push('Cmd');
  if (config.alt) names.push('Alt');
  if (config.shift) names.push('Shift');
  names.push(config.keyName.toUpperCase());
  return names.join(' + ');
}

// 监听按键：录制或触发截图
window.addEventListener('keydown', (e) => {
  // 如果处于录制模式，捕获按键组合
  if (isRecording) {
    e.preventDefault();
    e.stopPropagation();

    // 过滤掉纯修饰键本身的单独按下
    if (['Control', 'Alt', 'Shift', 'Meta'].includes(e.key)) {
      recordingText.textContent = '等待主按键 (如 A, S, X)...';
      return;
    }

    const recorded: HotkeyConfig = {
      ctrl: e.ctrlKey,
      alt: e.altKey,
      shift: e.shiftKey,
      meta: e.metaKey,
      code: e.code,
      keyName: e.key.length === 1 ? e.key.toUpperCase() : e.key,
    };

    tempRecordedHotkey = recorded;
    recordingText.textContent = formatHotkeyName(recorded);
    return;
  }

  // 正常运行模式：检测是否匹配当前激活的快捷键
  const matchModifier =
    Boolean(e.ctrlKey) === activeHotkey.ctrl &&
    Boolean(e.altKey) === activeHotkey.alt &&
    Boolean(e.shiftKey) === activeHotkey.shift &&
    Boolean(e.metaKey) === activeHotkey.meta;

  if (matchModifier && (e.code === activeHotkey.code || e.key.toUpperCase() === activeHotkey.keyName.toUpperCase())) {
    e.preventDefault();
    void doCapture();
  }
});

let isCapturing = false;

async function doCapture() {
  if (isCapturing) {
    logger.warn('已有截屏流程正在进行中，忽略重复调用');
    return;
  }
  isCapturing = true;
  logger.info('触发 doCapture 截屏流程');
  outputLog.textContent = '>> 正在调用 Tauri 原生 xcap 截图引擎捕获桌面...\n>> 收集全屏窗口候选并打开透明标注遮罩...';
  try {
    const result = await capture({
      locale: 'zh-CN',
      showCopyFeedback: true,
      defaultTool: 'select',
    });

    logger.info('Tauri capture 返回结果', { status: result.status, output: (result as any).output, bounds: (result as any).bounds });

    if (result.status === 'completed') {
      outputLog.textContent = [
        '✅ 截图标注已完成并写入系统剪贴板！',
        `• 动作: ${result.output.action}`,
        `• 选区尺寸: ${result.bounds.width} × ${result.bounds.height} (位置: x=${result.bounds.x}, y=${result.bounds.y})`,
        `• 图像大小: ${(result.data.length / 1024).toFixed(1)} KB`,
        `• 格式: ${result.mimeType}`,
        `• 显示器: ${result.displayId}`,
        '\n您可以直接在任何聊天软件或文档中 Ctrl+V 粘贴！'
      ].join('\n');
    } else if (result.status === 'cancelled') {
      showToast('ℹ️ 截图流程已取消', '已按 Esc 或点击取消退出', 2500);
      outputLog.textContent = 'ℹ️ 截图流程已取消 (按 Esc 或取消退出)';
    } else {
      showToast('❌ 截图未完成', `错误信息: ${(result as any).message || result.status}`, 3500);
      outputLog.textContent = `ℹ️ 截图状态: ${result.status}`;
      logger.warn('截图未完成或报错', { result });
    }
  } catch (err: any) {
    logger.error('Tauri 截图过程捕获到异常', err);
    showToast('❌ 截图异常', err?.message || String(err), 4000);
    outputLog.textContent = '❌ Tauri 截图异常: ' + (err?.message || String(err));
  } finally {
    isCapturing = false;
  }
}

/** 弹出右上角全局 Toast 提示语 */
function showToast(title: string, desc: string, duration = 3500) {
  const toast = document.getElementById('toastNotification');
  const titleEl = document.getElementById('toastTitle');
  const descEl = document.getElementById('toastDesc');
  if (!toast || !titleEl || !descEl) return;

  titleEl.textContent = title;
  descEl.textContent = desc;
  toast.classList.add('show');

  if ((toast as any)._timer) {
    clearTimeout((toast as any)._timer);
  }

  (toast as any)._timer = setTimeout(() => {
    toast.classList.remove('show');
  }, duration);
}

captureBtn.addEventListener('click', doCapture);
