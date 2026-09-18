# @open-snapora/tauri

> **Tauri (v2)** 애플리케이션을 위한 고성능 크로스 플랫폼 화면 캡처, 주석(어노테이션) 및 데스크톱 고정 툴킷.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/tauri.svg)](https://bundlephobia.com/package/@open-snapora/tauri)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/tauri.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://v2.tauri.app)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## 개요

`@open-snapora/tauri`는 Tauri v2 생태계를 위해 개발된 open-snapora의 공식 클라이언트 어댑터입니다. 네이티브 Rust 플러그인 [`tauri-plugin-snapora`](https://github.com/open-toolkits/open-snapora/tree/main/crates/tauri-plugin-snapora) 및 고성능 `xcap` 엔진을 기반으로 밀리초 단위의 멀티 모니터 화면 캡처, 대화형 캔버스 주석 오버레이, 드래그 및 크기 조절이 가능한 데스크톱 고정 플로팅 창(Pinned Window)을 가벼운 리소스로 제공합니다.

---

## 주요 기능

- ⚡ **네이티브 Rust 캡처 엔진**: `xcap` 하드웨어 가속을 통해 멀티 디스플레이 및 HiDPI 스케일링 환경에서도 깨짐 없이 빠른 캡처를 지원합니다.
- 🚀 **백그라운드 사전 예열 (`prewarm`)**: 앱 시작 시 투명 오버레이 창과 WebView2 런타임을 백그라운드에서 사전 로드하여 최초 캡처 시의 흰 화면 깜빡임과 지연을 완벽하게 제거합니다.
- 🪟 **스마트 윈도우 스내핑 (Window Snapping)**: 마우스 오버 시 활성 윈도우의 테두리를 자동으로 감지하고 하이라이트하여 한 번의 클릭으로 창 전체를 선택합니다.
- 🎨 **완벽한 주석 툴킷**:
  - **도형**: 직사각형, 타원, 화살표, 직선.
  - **그리기 및 모자이크**: 자유 브러시 및 민감한 정보를 가리기 위한 모자이크 / 블러 필터.
  - **텍스트 및 워터마크**: 크기, 색상, 외곽선, 배경 채우기를 지원하는 리치 텍스트 편집 및 워터마크 기능.
  - **작업 기록**: 무제한 단계의 실행 취소 및 다시 실행 (`Undo` / `Redo`).
  - **정밀 조절**: 컬러 피커(프리셋 팔레트 + HEX 입력), 선 두께 및 투명도 조절 슬라이더.
- 📌 **데스크톱 고정 (Pin to Desktop)**: 캡처 영역을 항상 위에 표시되는(Always-on-top) 테두리 없는 플로팅 창으로 데스크톱에 고정합니다. 드래그 이동, 리사이즈, 클립보드 재복사, 파일 저장 및 닫기를 지원합니다.
- 📋 **다양한 출력 파이프라인**:
  - 시스템 클립보드 복사 (PNG 형식, 복사 완료 플로팅 토스트 피드백 `showCopyFeedback` 지원).
  - 로컬 파일 저장 대화상자 (저장된 절대 경로 `filePath` 반환).
  - 데스크톱 고정 창 생성.
- 🌐 **다국어 기본 지원 (i18n)**: 한국어(`ko-KR`), 영어(`en-US`), 중국어 간체(`zh-CN`), 일본어(`ja-JP`), 스페인어(`es-ES`) 기본 탑재 및 개별 텍스트 재정의 지원.
- 🎨 **심층적인 비주얼 테마 설정**: 다크 / 라이트 모드 지원, 강조 색상, 마스크 불투명도 및 툴바 스타일을 자유롭게 커스터마이징할 수 있습니다.
- 🛡️ **Tauri v2 보안 규격 준수**: Tauri v2 Capabilities 권한 모델을 완벽하게 준수합니다.

---

## 설치 및 요구사항

### 1. 사전 요구사항

- **프론트엔드**: `@tauri-apps/api >= 2.0.0`
- **Rust 백엔드**: Tauri v2 (`tauri >= 2.0`), Rust `>= 1.77.2`

### 2. NPM 패키지 설치

```bash
npm install @open-snapora/tauri
# 또는
pnpm add @open-snapora/tauri
# 또는
yarn add @open-snapora/tauri
```

### 3. Rust 플러그인 추가

`src-tauri/Cargo.toml` 파일에 추가합니다:

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

---

## 빠른 시작

### 1단계: Capabilities 권한 설정

Tauri 설정 파일(예: `src-tauri/capabilities/default.json`)에 `snapora:default`를 등록합니다:

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

### 2단계: Rust에서 플러그인 등록

`src-tauri/src/lib.rs` (또는 `main.rs`)에서 등록합니다:

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("Tauri 애플리케이션 실행 실패");
}
```

### 3단계: 프론트엔드에서 캡처 호출

```typescript
import { capture, prewarm, type ScreenshotResult } from '@open-snapora/tauri';

// 앱 구동 시 백그라운드 사전 예열 (초기 캡처 시 즉각 응답)
void prewarm().catch(console.error);

async function handleTakeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      locale: 'ko-KR',
      showCopyFeedback: true,
      defaultTool: 'select',
      tools: ['rectangle', 'ellipse', 'arrow', 'brush', 'text', 'mosaic', 'watermark'],
      theme: {
        mode: 'dark',
        accentColor: '#3b82f6',
      },
    });

    if (result.status === 'completed') {
      console.log('캡처 성공!');
      console.log('선택된 액션:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('영역 좌표:', result.bounds);            // { x, y, width, height }
      console.log('디스플레이 ID:', result.displayId);
      console.log('PNG 바이트 수:', result.data.byteLength);

      if (result.output.action === 'save') {
        console.log('파일 저장 경로:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('사용자가 캡처를 취소했습니다 (Esc 키 또는 취소 버튼).');
    } else {
      console.error('캡처 실패:', result.code, result.message);
    }
  } catch (error) {
    console.error('예상치 못한 오류:', error);
  }
}
```

---

## 라이선스

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
