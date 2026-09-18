# @open-snapora/electron

> **Electron** 애플리케이션을 위한 고성능 멀티 디스플레이 화면 캡처, 주석(어노테이션) 및 데스크톱 고정(핀) 관리자.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/electron.svg)](https://bundlephobia.com/package/@open-snapora/electron)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/electron.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Electron >= 30](https://img.shields.io/badge/Electron-%3E%3D30.0.0-47848F.svg)](https://www.electronjs.org/)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## 개요

`@open-snapora/electron`은 Electron 데스크톱 앱을 위한 프로덕션 레벨의 멀티 모니터 화면 캡처, 대화형 주석 편집 및 데스크톱 고정 플로팅 창(Pinned Window) 솔루션입니다. 다중 모니터 간의 글로벌 좌표 변환과 HiDPI 스케일링을 정확하게 처리하며, 깜빡임(흰 화면 현상) 없는 투명 오버레이 윈도우 캐시 재사용 엔진 및 안전한 IPC 통신 아키텍처를 제공합니다.

---

## 주요 기능

- 🖥️ **네이티브 멀티 디스플레이 캡처**: Electron의 `desktopCapturer`를 기반으로 여러 화면의 물리 및 논리 좌표계를 통합하고 HiDPI 환경을 완벽하게 지원합니다.
- ⚡ **윈도우 캐시 재사용으로 지연 없는 실행**: 전체 화면 투명 오버레이 창을 백그라운드에서 캐시 및 재사용하여 반복 캡처 시 창 생성 오버헤드를 완벽히 제거합니다.
- 🪟 **스마트 윈도우 스내핑 (Window Snapping)**: 마우스 호버 시 앱 내의 표시 창 경계를 자동으로 감지하고 하이라이트합니다. OS 수준의 윈도우 좌표 목록을 주입하는 `getWindowSnapRegions`도 지원합니다.
- 📌 **독립적인 데스크톱 고정 창 (Pin to Desktop)**: 캡처 이미지를 항상 위에 표시(Always-on-top)되는 테두리 없는 독립 플로팅 창으로 고정합니다. 드래그 이동, 리사이즈, 클립보드 재복사, 파일 저장 및 자동 수명 주기 정리를 지원합니다.
- 🎨 **완벽한 캔버스 주석 도구 모음**:
  - **도형 도구**: 직사각형, 타원, 화살표, 직선.
  - **브러시 및 개인정보 보호**: 자유 브러시 및 강도 조절이 가능한 모자이크 / 블러 필터.
  - **텍스트 및 워터마크**: 글자 크기, 색상, 외곽선, 배경 채우기를 지원하는 리치 텍스트와 워터마크.
  - **작업 기록 관리**: 무제한 단계의 실행 취소 및 다시 실행 (`Undo` / `Redo`).
  - **스타일 제어**: 컬러 피커(프리셋 + HEX), 선 두께, 불투명도 조절.
- 🛡️ **엔터프라이즈급 보안 아키텍처**:
  - **Context Isolation**을 엄격히 준수하며 안전하지 않은 `remote` 모듈을 사용하지 않습니다.
  - `validateSender`를 통해 허가되지 않은 iframe 및 악의적인 IPC 호출을 사전에 차단합니다.
- 🚥 **동시성 및 대기열 제어 (`busyPolicy`)**: `'queue'`(순차 FIFO 대기열) 또는 `'reject'`(즉시 거부) 모드를 지원합니다.
- 🔌 **확장 가능한 출력 어댑터 (`outputAdapter`)**: 캡처된 버퍼를 로컬 디스크 대신 클라우드(AWS S3, OSS, 내부 API 등)로 직접 스트리밍할 수 있습니다.
- 📊 **단계별 마이크로초 단위 진단 (`onDiagnostic`)**: `capture` -> `overlay` -> `edit` -> `output` 각 단계별 소요 시간과 오류를 측정합니다.
- 🌐 **다국어 기본 지원 (i18n)**: 한국어(`ko-KR`), 영어(`en-US`), 중국어 간체(`zh-CN`), 일본어(`ja-JP`), 스페인어(`es-ES`) 탑재.

---

## 설치

```bash
npm install @open-snapora/electron
# 또는
pnpm add @open-snapora/electron
# 또는
yarn add @open-snapora/electron
```

> **필수 요구사항**: `electron >= 30.0.0`.

---

## 빠른 시작

### 1. 메인 프로세스 (`main.js` / `main.ts`)

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const path = require('path');
const { setupElectronSnapora, resolveHostPreloadPath } = require('@open-snapora/electron');

// 1. 메인 프로세스에서 Snapora 초기화
const snapora = setupElectronSnapora({
  busyPolicy: 'queue',
});

let mainWindow = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      preload: resolveHostPreloadPath(), // window.electronSnapora 안전 주입
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  mainWindow.loadFile('index.html');
}

app.whenReady().then(() => {
  createWindow();

  // 2. 글로벌 단축키 등록 (예: Ctrl+Alt+A / Command+Alt+A)
  const shortcut = process.platform === 'darwin' ? 'Command+Alt+A' : 'Ctrl+Alt+A';
  globalShortcut.register(shortcut, () => {
    snapora.manager.capture({
      locale: 'ko-KR',
      showCopyFeedback: true,
    });
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
  snapora.unregister();
});
```

### 2. 렌더러 프로세스 (`renderer.js`)

```typescript
async function handleTakeScreenshot() {
  try {
    const result = await window.electronSnapora.capture({
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
      console.log('캡처 완료!');
      console.log('실행 액션:', result.output.action);
      console.log('영역 좌표:', result.bounds);

      if (result.output.action === 'save') {
        console.log('저장된 파일 경로:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('캡처가 취소되었습니다.');
    } else {
      console.error('캡처 실패:', result.code, result.message);
    }
  } catch (error) {
    console.error('오류 발생:', error);
  }
}
```

---

## 데모 예제

[`demos/electron`](https://github.com/open-toolkits/open-snapora/tree/main/demos/electron) 디렉토리에서 완전히 동작하는 Electron 예제를 확인하실 수 있습니다.

---

## 라이선스

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
