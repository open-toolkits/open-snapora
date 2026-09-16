# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [1.0.6] - 2026-09-16

### Metadata & Search Visibility (元数据与开发者搜索能见度优化)

- **Comprehensive Keyword Expansion (关键词全场景扩充)**:
  - Greatly expanded search keywords across `@open-snapora/electron` and `@open-snapora/tauri` to 60+ targeted terms.
  - Enhanced searchability for window snapping (`window-snapping`, `smart-snapping`), desktop pinning (`pin-image`, `always-on-top`, `floating-window`), screen drawing & markup (`drawing-tool`, `screen-markup`, `mosaic`, `blur`), multi-monitor HiDPI (`hidpi`, `retina`, `per-monitor-dpi`), and framework integrations (`desktopCapturer`, `tauri-plugin-snapora`).

---

## [1.0.5] - 2026-09-16

### Architecture Refactoring & Stability Hardening (整体架构分层重构与稳定性强化)

#### 🚀 Architectural Layering & Decoupling (架构分层与核心解耦)
- **Session Lifecycle & ImageStore Decoupling**:
  - Completely detached heavy image byte buffers (`Vec<u8>`) from `ActiveSession`.
  - Introduced centralized `ImageStore` to independently manage raw screenshots (`source`) and exported annotations (`output`) by `job_id`.
  - Sessions now only store lightweight `ImageRef` and state machine contexts, preventing memory inflation and leaks.
- **Strict Job ID Lifecycle & 8-Stage State Machine**:
  - Implemented explicit session state transitions: `Created` → `Capturing` → `OverlayReady` → `Editing` → `Processing` → `Completed` / `Cancelled` / `Failed`.
  - Enforced strict `jobId` validation across all session commands (`confirm`, `cancel`, `report_error`, `output`), returning `Error::StaleSession` when receiving mismatched or stale asynchronous callbacks.
- **Application Service Layering**:
  - Extracted `CaptureService` to encapsulate complete screenshot lifecycle orchestration (capture, window snapping, session registration, overlay management, promise awaiting).
  - Simplified `commands/` into a thin IPC adapter layer without inlined business complexity.
- **Platform & Window Decoupling**:
  - Abstracted `ScreenCapture` and `WindowProvider` traits.
  - Extracted `SnapRegionCalculator` for window snapping heuristics.
  - Isolated OS-specific Win32 APIs (`EnumWindows`, `DwmGetWindowAttribute`, Per-Monitor V2 DPI awareness) into `platform/windows`.
- **Extensible Output Architecture**:
  - Introduced `OutputManager` and `OutputHandler` trait, making clipboard and disk file export modular and ready for upcoming OCR/AI/Upload extensions.
  - Decoupled `PinnedManager` from screenshot sessions, allowing pinned float windows to have completely independent lifecycles.

#### 🐛 Bug Fixes & Runtime Resilience (修复与运行鲁棒性)
- **Electron Host IPC Fallback**:
  - Fixed `TypeError: Cannot read properties of undefined (reading 'handle')` in `setupElectronSnapora` by defaulting to `electron.ipcMain` when not explicitly provided.
  - Supported top-level `busyPolicy` passing in `setupElectronSnapora`.
- **Tauri Vite Port Collision Auto-Healing**:
  - Added auto-detection and safe cleanup of orphan processes on port `1420` in `demos/tauri/scripts/sync-overlay.mjs`, eliminating `Error: Port 1420 is already in use` interruptions during local development.

---

## [1.0.4] - 2026-09-14

### Electron 44+ Clipboard Compatibility (剪贴板全版本兼容与异步安全)

- **Fix Electron 44+ Clipboard Removal**:
  - Electron 44 completely removed `clipboard.writeImage(image)`, which previously caused `EXPORT_FAILED` and prevented screenshot sessions from completing.
  - Implemented multi-tier compatibility strategy:
    - Primary: `clipboard.write({ image })` (officially recommended and supported across all Electron versions).
    - Fallback 1: `clipboard.writeImage(image)` (legacy Electron versions).
    - Fallback 2: `clipboard.writeBuffer('image/png', buffer)` (raw binary fallback).
  - Ensured all asynchronous clipboard write operations are properly `await`ed before returning completion status and tearing down screenshot sessions.

---

## [1.0.3] - 2026-09-14

### Improvements & Monorepo Tooling (优化与版本治理)

- **Unified Version Management**:
  - Added `pnpm run version:sync` and `pnpm run version:bump <ver>` to uniformly synchronize version across all subpackages (`packages/*`, `demos/*`, `Cargo.toml`).
- **Intelligent Publish Pipeline**:
  - `publish:packages` now pre-checks the npm registry and automatically skips versions that are already published, preventing pipeline interruptions.

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
