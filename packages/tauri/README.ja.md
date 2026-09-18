# @open-snapora/tauri

> **Tauri (v2)** 向けの高性能クロスプラットフォーム・デスクトップスクリーンショット、注釈（アノテーション）、およびデスクトップ固定（ピン留め）ツールキット。

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/tauri.svg)](https://bundlephobia.com/package/@open-snapora/tauri)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/tauri.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://v2.tauri.app)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## 概要

`@open-snapora/tauri` は、Tauri v2 アプリケーション向けに設計された open-snapora の公式クライアントアダプターです。ネイティブ Rust プラグイン [`tauri-plugin-snapora`](https://github.com/open-toolkits/open-snapora/tree/main/crates/tauri-plugin-snapora) および `xcap` キャプチャエンジンを搭載し、ミリ秒レベルの高速マルチディスプレイ画面キャプチャ、高機能な対話型 Canvas 注釈オーバーレイ、およびドラッグ・リサイズに対応したデスクトップ常駐ピン留めウィンドウ（Pinned Window）を最小限のリソースで提供します。

---

## 主な機能

- ⚡ **Rust ネイティブキャプチャエンジン**: `xcap` ハードウェアアクセラレーションによる高速画面キャプチャ。マルチディスプレイ環境や HiDPI スケーリングにも完全対応。
- 🚀 **バックグラウンド先行ウォームアップ (`prewarm`)**: アプリ起動時に透明オーバーレイウィンドウと WebView2 ランタイムをバックグラウンドで事前ロードし、初回スクリーンショット時のホワイトアウトや遅延を解消。
- 🪟 **ウィンドウ自動吸着 (Window Snapping)**: マウスホバー時にアクティブウィンドウの境界を自動検出し、ハイライト吸着。クリックするだけで対象ウィンドウを正確に選択可能。
- 🎨 **充実した注釈ツールキット**:
  - **図形**: 矩形（四角形）、楕円、矢印、直線。
  - **描画とプライバシー保護**: フリーハンドブラシ、個人情報や機密を隠すモザイク / ぼかしフィルター。
  - **テキストと透かし**: リッチテキスト編集（フォントサイズ、カラー、アウトライン、背景塗りつぶし）および半透明ウォーターマーク。
  - **履歴管理**: 制限なしのマルチステップ Undo（元に戻す） / Redo（やり直す）。
  - **細かなスタイル調整**: カラーピッカー（プリセットパレット + HEX入力）、線幅スライダー、不透明度設定。
- 📌 **デスクトップへのピン留め (Pin to Desktop)**: キャプチャした画像を最前面のフローティングウィンドウとしてデスクトップに固定。ドラッグ移動、サイズ変更、再クリップボードコピー、ローカル保存、閉じる操作に対応。
- 📋 **多彩な出力パイプライン**:
  - システムクリップボードへのコピー（PNG 形式、コピー成功時のトースト通知 `showCopyFeedback` 対応）。
  - ネイティブファイル保存ダイアログ（保存先絶対パス `filePath` を返却）。
  - デスクトップへのピン留めウィンドウ生成。
- 🌐 **多言語対応 (i18n)**: `ja-JP`（日本語）、`en-US`（英語）、`zh-CN`（簡体字中国語）、`ko-KR`（韓国語）、`es-ES`（スペイン語）を標準搭載。テキストの上書きカスタマイズも可能。
- 🎨 **高度なビジュアルテーマカスタマイズ**: ダーク / ライトモードの切り替え、アクセントカラー、マスク透明度、ツールバー色などの自由な設定。
- 🛡️ **Tauri v2 権限・セキュリティ仕様準拠**: Tauri v2 の Capabilities 権限モデルに厳密に準拠。

---

## インストールと要件

### 1. 前提条件

- **フロントエンド**: `@tauri-apps/api >= 2.0.0`
- **Rust バックエンド**: Tauri v2 (`tauri >= 2.0`), Rust `>= 1.77.2`

### 2. NPM パッケージのインストール

```bash
npm install @open-snapora/tauri
# または
pnpm add @open-snapora/tauri
# または
yarn add @open-snapora/tauri
```

### 3. Rust プラグインの追加

Tauri プロジェクトの `src-tauri/Cargo.toml` に追加します：

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

---

## クイックスタート

### ステップ 1: Capabilities の設定

Tauri のケーパビリティ設定（例: `src-tauri/capabilities/default.json`）に `snapora:default` を追加します：

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

### ステップ 2: Rust 側でプラグインを登録

`src-tauri/src/lib.rs`（または `main.rs`）でプラグインを登録します：

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("Tauri アプリの実行に失敗しました");
}
```

### ステップ 3: フロントエンドからキャプチャを実行

```typescript
import { capture, prewarm, type ScreenshotResult } from '@open-snapora/tauri';

// アプリ起動時にオーバーレイを先行ウォームアップ（ミリ秒級の高速起動を実現）
void prewarm().catch(console.error);

async function handleTakeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      locale: 'ja-JP',
      showCopyFeedback: true,
      defaultTool: 'select',
      tools: ['rectangle', 'ellipse', 'arrow', 'brush', 'text', 'mosaic', 'watermark'],
      theme: {
        mode: 'dark',
        accentColor: '#3b82f6',
      },
    });

    if (result.status === 'completed') {
      console.log('キャプチャ成功！');
      console.log('実行アクション:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('選択範囲:', result.bounds);              // { x, y, width, height }
      console.log('ディスプレイID:', result.displayId);
      console.log('画像データバイト長:', result.data.byteLength);

      if (result.output.action === 'save') {
        console.log('保存先ファイルパス:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('ユーザーによってキャンセルされました (Escキーまたはキャンセルボタン)。');
    } else {
      console.error('キャプチャ失敗:', result.code, result.message);
    }
  } catch (error) {
    console.error('予期しないエラー:', error);
  }
}
```

---

## API リファレンス

### メソッド

#### `capture(options?: ScreenshotOptions): Promise<ScreenshotResult>`

全画面キャプチャセッションを開始し、注釈オーバーレイを表示します。ユーザーの確定、キャンセル、またはエラー終了まで待機します。

#### `cancel(): Promise<boolean>`

実行中のキャプチャセッションをプログラムからキャンセルします。アクティブなセッションが存在した場合は `true` を返します。

#### `prewarm(): Promise<void>`

非表示の透明オーバーレイウィンドウと WebView2 ランタイムをバックグラウンドでサイレント初期化します。

#### `createTauriOverlayBridge(): ScreenshotOverlayApi`

カスタム注釈オーバーレイ UI を独自開発するための低レベルブリッジファクトリ。

#### `createTauriPinnedBridge(): PinnedImageApi`

カスタムピン留めウィンドウ UI を独自開発するための低レベルブリッジファクトリ。

---

### 型定義

#### `ScreenshotOptions`

```typescript
export interface ScreenshotOptions {
  /** 対象ディスプレイ: 'cursor' (既定), 'primary', または特定の displayId */
  display?: 'cursor' | 'primary' | string;

  /** ツールバーに表示する注釈ツールの配列 */
  tools?: ScreenshotTool[];

  /** 起動時にデフォルトで選択されるツール */
  defaultTool?: 'select' | ScreenshotTool;

  /** クリップボードコピー成功時にトースト通知を表示するかどうか (既定: false) */
  showCopyFeedback?: boolean;

  /** 言語コード: 'ja-JP' | 'en-US' | 'zh-CN' | 'ko-KR' | 'es-ES' */
  locale?: ScreenshotLocale;

  /** UI文言の上書き設定 */
  messages?: ScreenshotMessageOverrides;

  /** ビジュアルテーマ設定 */
  theme?: ScreenshotTheme;
}
```

#### `ScreenshotResult`

```typescript
export type ScreenshotResult =
  | (ScreenshotImageResult & { output: ScreenshotOutputMetadata })
  | { status: 'cancelled' }
  | { status: 'failed'; code: ScreenshotErrorCode; message: string };

export interface ScreenshotImageResult {
  status: 'completed';
  data: Uint8Array; // PNG バイト配列
  mimeType: 'image/png';
  bounds: ScreenshotBounds; // { x, y, width, height }
  displayId: string;
}

export type ScreenshotOutputMetadata =
  | { action: 'copy' }
  | { action: 'save'; filePath: string }
  | { action: 'pin' };
```

---

## 応用例

### 1. グローバルショートカットの登録

`@tauri-apps/plugin-global-shortcut` を利用して、グローバルショートカットで呼び出すことができます：

```typescript
import { register } from '@tauri-apps/plugin-global-shortcut';
import { capture } from '@open-snapora/tauri';

await register('CommandOrControl+Shift+A', async (event) => {
  if (event.state === 'Pressed') {
    await capture({ showCopyFeedback: true });
  }
});
```

---

## デモサンプル

動作可能な Tauri v2 サンプルは [`demos/tauri`](https://github.com/open-toolkits/open-snapora/tree/main/demos/tauri) ディレクトリで確認できます。

---

## ライセンス

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
