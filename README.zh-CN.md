# Open Snapora

> 跨框架桌面端屏幕截图与图像标注开发套件，支持 **Tauri (v2)** 与 **Electron**。

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

---

## 特性

- ⚡ **原生级截屏**：Tauri (Rust `xcap`) 与 Electron (`desktopCapturer`) 高性能多屏捕获。
- 🎨 **丰富标注工具**：矩形、椭圆、箭头、画笔、马赛克、文字、撤销/重做、取色器。
- 📌 **桌面贴图**：支持将截图钉在桌面上自由拖拽、缩放与置顶展示。
- 🖥️ **多屏与 HiDPI**：自动适应多显示器坐标映射与高分屏缩放。

---

## Tauri (v2) 集成

### 1. 安装依赖

```bash
npm install @open-snapora/tauri
# 或
pnpm add @open-snapora/tauri
```

### 2. 添加 Rust 插件

在 `src-tauri/Cargo.toml` 中添加依赖：

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

### 3. 配置权限

在 `src-tauri/capabilities/default.json` 中配置：

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "identifier": "default",
  "windows": ["*"],
  "permissions": [
    "core:default",
    "snapora:default"
  ]
}
```

### 4. 注册插件

在 `src-tauri/src/main.rs` 中：

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 5. 调用截屏

```typescript
import { capture, cancel, type ScreenshotResult } from '@open-snapora/tauri';

async function takeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      hideOwnerWindow: true, // 截屏前自动隐藏当前窗口
    });

    if (result.status === 'completed') {
      console.log('图片二进制:', result.data);
      console.log('选区坐标:', result.bounds);
      console.log('输出结果:', result.output);
    }
  } catch (error) {
    console.error('截屏失败:', error);
  }
}
```

### 6. 打包发布 (Windows .exe)

```bash
tauri build
```

- 若需独立免安装 `.exe`：在 `tauri.conf.json` 中配置 `"bundle": { "active": false }`。
- 若需安装包 (NSIS / MSI)：在 `tauri.conf.json` 中配置 `"bundle": { "active": true, "targets": ["nsis", "msi"] }`。

---

## Electron 集成

### 1. 安装依赖

```bash
npm install @open-snapora/electron
# 或
pnpm add @open-snapora/electron
```

### 2. 主进程初始化

在 Electron 主进程脚本中：

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const { setupSnaporaMain } = require('@open-snapora/electron');

let snapora = null;

app.whenReady().then(() => {
  snapora = setupSnaporaMain({
    onComplete: (data) => {
      console.log('截屏完成:', data);
    },
    onCancel: (reason) => {
      console.log('截屏取消:', reason);
    },
  });

  globalShortcut.register('CommandOrControl+Shift+A', () => {
    snapora.startCapture();
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
});
```

---

## API 参考

### `capture(options?: ScreenshotOptions): Promise<ScreenshotResult>`

| 参数 | 类型 | 默认值 | 说明 |
| :--- | :--- | :--- | :--- |
| `hideOwnerWindow` | `boolean` | `false` | 截屏前是否自动隐藏调用窗口 |

### `ScreenshotResult`

```typescript
type ScreenshotResult =
  | {
      status: 'completed';
      data: Uint8Array;
      mimeType: 'image/png';
      bounds: { x: number; y: number; width: number; height: number };
      displayId: string;
      output: { action: 'copy' } | { action: 'save'; filePath: string } | { action: 'pin' };
    }
  | { status: 'cancelled' }
  | { status: 'failed'; code?: string; message?: string };
```

---

## 示例工程

完整实现示例可参考仓库 `demos/` 目录：
- `demos/tauri/`：Tauri (v2) 集成示例
- `demos/electron/`：Electron 集成示例

---

## 开源协议

[MIT License](./LICENSE)
