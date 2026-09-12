# @open-snapora/overlay

> Host-agnostic Web screenshot annotation and overlay editor for open-snapora.

[![npm version](https://img.shields.io/npm/v/@open-snapora/overlay.svg)](https://www.npmjs.com/package/@open-snapora/overlay)
[![license](https://img.shields.io/npm/l/@open-snapora/overlay.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)

---

## 📦 Installation

```bash
npm install @open-snapora/overlay
# or
pnpm add @open-snapora/overlay
```

---

## 🚀 Features

- 🖥️ **Pure Webview / Canvas Overlay**: High-performance interactive selection, drag handles, and magnifying loupe.
- 🎨 **Full Annotation Toolkit**:
  - Shapes: Rectangle, Ellipse, Arrow, Freehand Brush, Mosaic/Blur.
  - Text: Editable inline text with stroke and font size adjustments.
  - Controls: Undo, Redo, Color Palette, Thickness selector.
- 📌 **Pin to Desktop**: Built-in floating pinned viewer with dragging, zooming, copying, and saving support.
- 🔌 **Host Decoupled**: Connects to any desktop host (Electron, Tauri, Wry, CEF, WebView2, or pure browser) via standard bridge contracts.

---

## 💡 Quick Usage

### Mount in Custom Webview

```typescript
import { mountOverlay } from '@open-snapora/overlay';
import type { OverlayBridge } from '@open-snapora/shared';

const bridge: OverlayBridge = {
  getInitState: async () => ({
    frame: {
      display: { id: '1', bounds: { x: 0, y: 0, width: 1920, height: 1080 }, scaleFactor: 1 },
      pixel_size: { width: 1920, height: 1080 },
    },
    language: 'zh-CN',
  }),
  ready: () => console.log('Overlay ready'),
  confirm: (payload) => console.log('Confirmed:', payload),
  cancel: (reason) => console.log('Canceled:', reason),
  pin: (payload) => console.log('Pin to desktop:', payload),
};

const app = mountOverlay({
  container: document.getElementById('app')!,
  bridge,
  loadFrame: async (screenId) => {
    // Provide your screen capture frame (ImageData / URL / Bitmap)
    return myNativeGetFrame(screenId);
  },
});
```

---

## 📄 License

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
