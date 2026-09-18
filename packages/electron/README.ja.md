# @open-snapora/electron

> **Electron** アプリケーション向けの高機能マルチディスプレイスクリーンショット、注釈（アノテーション）、およびデスクトップ固定（ピン留め）マネージャー。

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/electron.svg)](https://bundlephobia.com/package/@open-snapora/electron)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/electron.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Electron >= 30](https://img.shields.io/badge/Electron-%3E%3D30.0.0-47848F.svg)](https://www.electronjs.org/)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## 概要

`@open-snapora/electron` は、Electron デスクトップアプリに商用レベルのマルチモニタキャプチャ、注釈編集、およびデスクトップ常駐ピン留めウィンドウ機能を提供します。複数ディスプレイにわたるグローバル座標の自動変換や Retina/HiDPI スケーリングに対応し、チラつき（ホワイトアウト）のない透明オーバーレイウィンドウのキャッシュ再利用機構とセキュアな IPC 通信レイヤーを備えています。

---

## 主な機能

- 🖥️ **ネイティブマルチディスプレイキャプチャ**: Electron の `desktopCapturer` をベースに、複数画面の物理・論理座標系を自動統合し、HiDPI スケーリングにも完全対応。
- ⚡ **ゼロ遅延・ウィンドウキャッシュ再利用**: 全画面透明オーバーレイウィンドウをキャッシュ・再利用することで、初回および連続キャプチャ時のウィンドウ生成オーバーヘッドを排除。
- 🪟 **ウィンドウ自動吸着 (Window Snapping)**: ホバー時にアプリ内の表示ウィンドウ境界を自動検出してハイライト吸着。OS レベルのネイティブウィンドウ座標を注入する `getWindowSnapRegions` もサポート。
- 📌 **独立したピン留めウィンドウ (Pin to Desktop)**: キャプチャ画像を常に最前面表示（Always-on-top）のフローティングウィンドウとして固定。ドラッグ移動、サイズ変更、再コピー、保存、自動ライフサイクル管理を完備。
- 🎨 **高機能な注釈ツールキット**:
  - **図形**: 矩形（四角形）、楕円、矢印、直線。
  - **フリーハンドとプライバシー保護**: ブラシ描画、強度調整可能なモザイク / ぼかしフィルター。
  - **テキストと透かし**: 文字サイズ、色、枠線、背景塗りつぶしに対応したテキスト入力および半透明ウォーターマーク。
  - **履歴操作**: 無制限の Undo（元に戻す） / Redo（やり直す）。
  - **細かなスタイル調整**: カラーピッカー（プリセット + HEX）、線幅、不透明度。
- 🛡️ **堅牢なセキュリティ設計**:
  - **Context Isolation**（コンテキスト分離）に完全準拠。安全でない `remote` モジュールは一切不使用。
  - `validateSender` による呼び出し元検証で、不正な iframe や不正 IPC リクエストを自動遮断。
- 🚥 **同時実行キューイング制御 (`busyPolicy`)**: `'queue'`（FIFO で後続リクエストを自動整列）または `'reject'`（ビジー時即時拒否）を選択可能。
- 🔌 **柔軟な出力パイプライン (`outputAdapter`)**: 出力処理をフックして、キャプチャ画像を直接クラウド（AWS S3、OSS、社内APIなど）へアップロード可能。
- 📊 **フェーズ別マイクロ秒診断 (`onDiagnostic`)**: `capture` -> `overlay` -> `edit` -> `output` の各フェーズごとの処理時間とエラーログを正確に記録。
- 🌐 **多言語対応 (i18n)**: 日本語（`ja-JP`）、英語（`en-US`）、簡体字中国語（`zh-CN`）、韓国語（`ko-KR`）、スペイン語（`es-ES`）を標準搭載。
- 🎨 **テーマカスタマイズ**: ダーク / ライトモード、アクセントカラー、ツールバー色などの設定に対応。

---

## インストール

```bash
npm install @open-snapora/electron
# または
pnpm add @open-snapora/electron
# または
yarn add @open-snapora/electron
```

> **要件**: `electron >= 30.0.0` が必要です。

---

## クイックスタート（ワンストップ統合）

### 1. メインプロセスの設定 (`main.js` / `main.ts`)

`setupElectronSnapora` を呼び出すだけで、マネージャー生成、IPC ハンドラー登録、標準 Preload スクリプトの解決が完了します：

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const path = require('path');
const { setupElectronSnapora, resolveHostPreloadPath } = require('@open-snapora/electron');

// 1. メインプロセスで Snapora コントローラーを初期化
const snapora = setupElectronSnapora({
  busyPolicy: 'queue', // キャプチャ処理中の重複リクエストを自動キューイング
});

let mainWindow = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      preload: resolveHostPreloadPath(), // 安全に window.electronSnapora を注入
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  mainWindow.loadFile('index.html');
}

app.whenReady().then(() => {
  createWindow();

  // 2. グローバルショートカットを登録（例: Ctrl+Alt+A / Command+Alt+A）
  const shortcut = process.platform === 'darwin' ? 'Command+Alt+A' : 'Ctrl+Alt+A';
  globalShortcut.register(shortcut, () => {
    snapora.manager.capture({
      locale: 'ja-JP',
      showCopyFeedback: true,
    });
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
  snapora.unregister(); // リソース解放
});
```

### 2. レンダラープロセスでの呼び出し (`renderer.js` など)

```typescript
async function handleTakeScreenshot() {
  try {
    const result = await window.electronSnapora.capture({
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
      console.log('キャプチャ完了！');
      console.log('実行アクション:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('選択範囲:', result.bounds);
      console.log('ディスプレイID:', result.displayId);

      if (result.output.action === 'save') {
        console.log('保存先ファイルパス:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('ユーザーによってキャンセルされました。');
    } else {
      console.error('キャプチャ失敗:', result.code, result.message);
    }
  } catch (error) {
    console.error('エラーが発生しました:', error);
  }
}
```

---

## デモサンプル

完全な Electron サンプルプロジェクトは [`demos/electron`](https://github.com/open-toolkits/open-snapora/tree/main/demos/electron) ディレクトリでご確認いただけます。

---

## ライセンス

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
