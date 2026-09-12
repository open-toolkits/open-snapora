# @open-snapora/electron

> Electron host adapter and multi-screen screenshot manager for open-snapora.

[![npm version](https://img.shields.io/npm/v/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![license](https://img.shields.io/npm/l/@open-snapora/electron.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)

---

## Installation

```bash
npm install @open-snapora/electron
# or
pnpm add @open-snapora/electron
```

> **Peer Dependency**: Requires `electron >= 30.0.0`.

---

## Features

- 🖥️ **Multi-Display Capture**: Smoothly captures all connected screens and maps coordinate spaces across displays.
- ⚡ **Prewarm & Fast Startup**: Pre-warms transparent overlay windows in the background to achieve instantaneous capture response.
- 📌 **Pin to Desktop**: Native floating pinned window support with full drag, copy, and save capabilities.
- 🛡️ **Secure Preload**: Safe IPC bridge between Electron main process and overlay webview renderer.

---

## Usage

### In Electron Main Process

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const { setupSnaporaMain } = require('@open-snapora/electron');

let snaporaController = null;

app.whenReady().then(() => {
  // 1. Initialize Snapora controller
  snaporaController = setupSnaporaMain({
    onComplete: (data) => {
      console.log('Capture completed:', data);
    },
    onCancel: (reason) => {
      console.log('Capture canceled:', reason);
    },
  });

  // 2. Register global shortcut to trigger capture
  globalShortcut.register('CommandOrControl+Shift+A', () => {
    snaporaController.startCapture();
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
});
```

### TypeScript Types

All types and contracts are exported directly from `@open-snapora/electron`:

```typescript
import {
  setupSnaporaMain,
  type ScreenshotResult,
  type ScreenshotOptions,
  type ScreenshotJobContext,
} from '@open-snapora/electron';
```

---

## License

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
