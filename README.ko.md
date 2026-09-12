# Open Snapora

> **Tauri (v2)** 및 **Electron** 데스크톱 애플리케이션을 위한 화면 캡처 & 이미지 주석 개발 툴킷.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

---

## 주요 기능

- ⚡ **네이티브 캡처**: Tauri (Rust `xcap`) 및 Electron (`desktopCapturer`) 기반의 고성능 멀티 모니터 캡처.
- 🎨 **주석 도구 모음**: 사각형, 원형, 화살표, 브러시, 모자이크, 텍스트 입력, 실행 취소/다시 실행, 스포이트.
- 📌 **바탕화면 핀 고정**: 캡처 영역을 드래그 및 크기 조절 가능한 항상 위 플로팅 핀 창으로 띄우기.
- 🖥️ **다중 모니터 & HiDPI**: 멀티 디스플레이 좌표 자동 매핑 및 고해상도 DPI 스케일링 대응.

---

## Tauri (v2) 연동 가이드

### 1. 패키지 설치

```bash
npm install @open-snapora/tauri
# 또는
pnpm add @open-snapora/tauri
```

### 2. Rust 플러그인 추가

`src-tauri/Cargo.toml` 파일에 추가:

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

### 3. 권한(Capabilities) 설정

`src-tauri/capabilities/default.json`에 추가:

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

### 4. 플러그인 등록

`src-tauri/src/main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 5. 화면 캡처 호출

```typescript
import { capture, cancel, type ScreenshotResult } from '@open-snapora/tauri';

async function takeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      hideOwnerWindow: true, // 캡처 시 호출 창 자동 숨김
    });

    if (result.status === 'completed') {
      console.log('이미지 데이터:', result.data);
      console.log('선택 영역:', result.bounds);
      console.log('출력 액션:', result.output);
    }
  } catch (error) {
    console.error('캡처 실패:', error);
  }
}
```

### 6. 릴리스 빌드 (Windows .exe)

```bash
tauri build
```

- **단일 무설치 포터블 `.exe`**: `tauri.conf.json`에서 `"bundle": { "active": false }`로 설정.
- **설치 프로그램 (NSIS / MSI)**: `tauri.conf.json`에서 `"bundle": { "active": true, "targets": ["nsis", "msi"] }`로 설정.

---

## Electron 연동 가이드

### 1. 패키지 설치

```bash
npm install @open-snapora/electron
# 또는
pnpm add @open-snapora/electron
```

### 2. 메인 프로세스 초기화

메인 프로세스 스크립트에서:

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const { setupSnaporaMain } = require('@open-snapora/electron');

let snapora = null;

app.whenReady().then(() => {
  snapora = setupSnaporaMain({
    onComplete: (data) => {
      console.log('캡처 완료:', data);
    },
    onCancel: (reason) => {
      console.log('캡처 취소됨:', reason);
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

## API 레퍼런스

### `capture(options?: ScreenshotOptions): Promise<ScreenshotResult>`

| 옵션 | 타입 | 기본값 | 설명 |
| :--- | :--- | :--- | :--- |
| `hideOwnerWindow` | `boolean` | `false` | 캡처 전 호출자 창 자동 숨김 여부 |

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

## 예제 프로젝트

구현 참고를 위해 저장소의 `demos/` 디렉터리를 확인하세요:
- `demos/tauri/`: Tauri (v2) 연동 예제
- `demos/electron/`: Electron 연동 예제

---

## 라이선스

[MIT License](./LICENSE)
