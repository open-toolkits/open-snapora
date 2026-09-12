# Open Snapora

> **Tauri (v2)** および **Electron** 向けデスクトップスクリーンショット＆画像注釈ツールキット。

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

---

## 主な機能

- ⚡ **高速ネイティブキャプチャ**：Tauri (Rust `xcap`) および Electron (`desktopCapturer`) によるマルチディスプレイ対応。
- 🎨 **注釈ツール**：矩形、楕円、矢印、ブラシ、モザイク、文字ツール、Undo/Redo、カラーピッカー。
- 📌 **デスクトップピン留め**：切り抜いた画像をドラッグ・拡大縮小可能な最前面ウィンドウとして固定表示。
- 🖥️ **マルチモニタ & HiDPI**：複数ディスプレイ間の座標自動マッピングと高DPIスケーリングに対応。

---

## Tauri (v2) 導入手順

### 1. 依存関係のインストール

```bash
npm install @open-snapora/tauri
# または
pnpm add @open-snapora/tauri
```

### 2. Rust プラグインの追加

`src-tauri/Cargo.toml` に追加：

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

### 3. パーミッションの設定

`src-tauri/capabilities/default.json` に設定：

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

### 4. プラグインの登録

`src-tauri/src/main.rs` にて：

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 5. キャプチャの呼び出し

```typescript
import { capture, cancel, type ScreenshotResult } from '@open-snapora/tauri';

async function takeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      hideOwnerWindow: true, // キャプチャ時に自ウィンドウを非表示にする
    });

    if (result.status === 'completed') {
      console.log('画像データ:', result.data);
      console.log('選択範囲:', result.bounds);
      console.log('出力形式:', result.output);
    }
  } catch (error) {
    console.error('キャプチャ失敗:', error);
  }
}
```

### 6. ビルド (Windows .exe)

```bash
tauri build
```

- **単一ポータブル `.exe`**：`tauri.conf.json` で `"bundle": { "active": false }` と設定。
- **インストーラー (NSIS / MSI)**：`tauri.conf.json` で `"bundle": { "active": true, "targets": ["nsis", "msi"] }` と設定。

---

## Electron 導入手順

### 1. 依存関係のインストール

```bash
npm install @open-snapora/electron
# または
pnpm add @open-snapora/electron
```

### 2. メインプロセスの初期化

メインプロセススクリプトにて：

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const { setupSnaporaMain } = require('@open-snapora/electron');

let snapora = null;

app.whenReady().then(() => {
  snapora = setupSnaporaMain({
    onComplete: (data) => {
      console.log('キャプチャ完了:', data);
    },
    onCancel: (reason) => {
      console.log('キャンセル:', reason);
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

## API リファレンス

### `capture(options?: ScreenshotOptions): Promise<ScreenshotResult>`

| オプション | 型 | デフォルト値 | 説明 |
| :--- | :--- | :--- | :--- |
| `hideOwnerWindow` | `boolean` | `false` | キャプチャ前に呼び出し元ウィンドウを自動的に隠すか |

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

## デモプロジェクト

実装例は `demos/` ディレクトリを参照してください：
- `demos/tauri/`：Tauri (v2) 連携例
- `demos/electron/`：Electron 連携例

---

## ライセンス

[MIT License](./LICENSE)
