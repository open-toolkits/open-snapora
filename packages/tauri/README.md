# @open-snapora/tauri

> Tauri v2 host adapter and frontend bindings for open-snapora.

[![npm version](https://img.shields.io/npm/v/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![license](https://img.shields.io/npm/l/@open-snapora/tauri.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)

---

## Installation

```bash
npm install @open-snapora/tauri
# or
pnpm add @open-snapora/tauri
```

> **Requirements**:
> - Frontend: `@tauri-apps/api >= 2.0.0`
> - Rust backend: `tauri-plugin-snapora` (in `src-tauri/Cargo.toml`)

---

## Features

- ⚡ **Native Rust Capture Engine**: Works with `tauri-plugin-snapora` (powered by `xcap`) for fast multi-monitor capture.
- 🪟 **Tauri v2 Window Management**: Transparent overlay creation, positioning, and pinned window dragging.
- 🎯 **Promise-based API**: Single `capture()` function.

---

## Usage

### 1. In Tauri Frontend

```typescript
import { capture, cancel, type ScreenshotResult, type ScreenshotOptions } from '@open-snapora/tauri';

async function onScreenshotClick() {
  try {
    const result: ScreenshotResult = await capture({
      hideOwnerWindow: true,
    });

    if (result.status === 'completed') {
      console.log('Capture succeeded:', result);
    }
  } catch (err) {
    console.error('Capture failed:', err);
  }
}
```

### 2. In Tauri Rust Backend (`src-tauri/Cargo.toml`)

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

### 3. Register Plugin (`src-tauri/src/main.rs`)

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("Failed to run app");
}
```

---

## License

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
