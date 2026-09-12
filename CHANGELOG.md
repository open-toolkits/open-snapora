# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.2] - 2026-09-12

### Architecture Refactoring & Pure Single-Package Publishing (单包架构重构与独立分发)

- **Pure Self-Contained Bundle**:
  - `@open-snapora/electron` and `@open-snapora/tauri` now bundle and inline all shared core logic and TypeScript types via `tsup` (`noExternal: ['@open-snapora/shared']`).
  - Consumers no longer need to install `@open-snapora/shared` or `@open-snapora/overlay` separately.
- **Internal Private Packages**:
  - Marked `@open-snapora/shared` and `@open-snapora/overlay` as `"private": true`, keeping npm registry clean with only user-facing SDKs.

---

## [1.0.1] - 2026-09-12

### Initial Release (初始版本发布)

#### 🚀 Core & Architecture (核心与架构)
- **Host-Decoupled Design**: Completely separated core geometry calculation, document models, and undo/redo stacks from desktop containers (Electron, Tauri v2, Webview).
- **Multi-Monitor Support**: Seamless coordinate translation across multiple connected displays, with full HiDPI / Retina auto-adaptation.
- **Pin to Desktop**: Built-in desktop pinning window enabling users to float, drag, resize, and pin annotated clips always-on-top.
- **Strict Protocol Contracts**: Typed `BridgeMessage` validation protecting host privileges from untrusted renderer calls.
- **Built-in i18n**: Out-of-the-box multi-language support for 5 major languages: English, Simplified Chinese, Japanese, Spanish, and Korean.

#### 📦 Packages (发布的 NPM 包)
- **`@open-snapora/shared`**:
  - Core geometry operations (`normalizeRect`, `isPointInRect`, `scaleRect`, coordinate transformation).
  - Immutable document history stack with full Undo / Redo capabilities.
  - IPC protocol schemas, message contracts, and payload validators.
  - Internationalization dictionaries (`en-US`, `zh-CN`, `ja-JP`, `es-ES`, `ko-KR`).
- **`@open-snapora/overlay`**:
  - High-performance Canvas/DOM annotation editor.
  - Full toolkit: Rectangle, Ellipse, Arrow, Freehand Brush, Mosaic/Pixelate, Text, Color Palette, and Loupe magnifier.
  - Standalone Pin-to-Desktop viewer with copy, save, and custom context menu.
- **`@open-snapora/electron`**:
  - Electron host adapter with multi-screen capture manager (`setupSnaporaMain`).
  - Seamless support for `desktopCapturer` stream capture and off-screen compositing.
  - Secure preload scripts and re-exported shared types for single-package integration.
- **`@open-snapora/tauri`**:
  - Tauri (v2) frontend SDK and bindings for native screenshot capture.
  - Bundled overlay static assets in package distribution.
  - Built-in CLI `open-snapora-tauri sync` to copy overlay assets into user projects.
- **`tauri-plugin-snapora` (Rust Crate)**:
  - High-performance native Rust plugin powered by `xcap` for concurrent multi-display frame capture.
  - Transparent, borderless, frameless fullscreen overlay window lifecycle management.
  - High-resolution physical pixel coordinate alignment.

#### 🔨 Demos & Windows .exe Build
- Included complete working reference applications:
  - `demos/electron`: Full-screen multi-screen capture in Electron.
  - `demos/tauri`: Lightning-fast capture with Tauri v2 + Rust.
- Verified standalone Windows `.exe` build (~10.3 MB single portable binary) without external asset dependencies.
