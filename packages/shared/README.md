# @open-snapora/shared

> Shared core models, geometry algorithms, history undo/redo stacks, protocol contracts, and i18n for open-snapora.

[![npm version](https://img.shields.io/npm/v/@open-snapora/shared.svg)](https://www.npmjs.com/package/@open-snapora/shared)
[![license](https://img.shields.io/npm/l/@open-snapora/shared.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)

---

## 📦 Installation

```bash
npm install @open-snapora/shared
# or
pnpm add @open-snapora/shared
# or
yarn add @open-snapora/shared
```

---

## 🚀 Features

- 📐 **Geometry Engine**: Point, Rect, Bounds calculation, HiDPI scaling, and multi-display coordinate mapping.
- 🎨 **Annotation Models**: Rectangles, ellipses, arrows, freehand paths, mosaic pixelation, and rich text annotation schemas.
- ⏪ **Undo/Redo History**: Standardized immutable state management for annotation actions.
- 🔒 **Protocol Contracts**: Typed IPC bridge messages (`BridgeMessage`, `HostFrame`, `InitPayload`) ensuring host-agnostic communication between webview and desktop containers.
- 🌍 **Built-in i18n**: Out-of-the-box support for English, Simplified Chinese, Japanese, Spanish, and Korean.

---

## 💡 Quick Usage

### Geometry & Bounding Box

```typescript
import { normalizeRect, isPointInRect, scaleRect } from '@open-snapora/shared';

const rect = normalizeRect({ x: 200, y: 300, width: -100, height: -150 });
// => { x: 100, y: 150, width: 100, height: 150 }

const inside = isPointInRect({ x: 120, y: 180 }, rect);
// => true
```

### Protocol Messages

```typescript
import type { BridgeMessage, InitPayload } from '@open-snapora/shared/protocol';

function handleMessage(msg: BridgeMessage) {
  if (msg.type === 'confirm') {
    console.log('User confirmed selection and annotations:', msg.payload);
  }
}
```

### Multi-language (i18n)

```typescript
import { getI18nTexts, setLanguage } from '@open-snapora/shared/i18n';

// Supports 'en-US' | 'zh-CN' | 'ja-JP' | 'es-ES' | 'ko-KR'
const t = getI18nTexts('zh-CN');
console.log(t.toolbar.rect); // "矩形"
```

---

## 📄 License

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
