const { app, BrowserWindow, globalShortcut } = require('electron');
const path = require('path');
const { setupElectronSnapora, resolveHostPreloadPath } = require('@open-snapora/electron');

// 1. 初始化并注册 open-snapora 插件
const snapora = setupElectronSnapora({
  busyPolicy: 'queue', // 当截图正在进行中时采用排队策略
});

let mainWindow = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 900,
    height: 650,
    title: 'open-snapora Electron 集成演示',
    icon: path.join(__dirname, 'assets/icon.png'),
    webPreferences: {
      preload: resolveHostPreloadPath(), // 注入标准宿主 Preload (window.electronSnapora)
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  mainWindow.loadFile(path.join(__dirname, 'index.html'));

  mainWindow.on('closed', () => {
    mainWindow = null;
  });
}

app.whenReady().then(() => {
  createWindow();

  // 2. 注册可选的全局截图快捷键 Ctrl+Alt+A (macOS 为 Command+Alt+A)
  const shortcutKey = process.platform === 'darwin' ? 'Command+Alt+A' : 'Ctrl+Alt+A';
  globalShortcut.register(shortcutKey, () => {
    if (mainWindow && !mainWindow.isDestroyed()) {
      mainWindow.webContents.send('trigger-screenshot');
    }
  });

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow();
    }
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});
