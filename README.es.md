# Open Snapora

> Suite de captura de pantalla y anotación de imágenes para aplicaciones de escritorio con **Tauri (v2)** y **Electron**.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

---

## Características

- ⚡ **Captura Nativa**: Acelerada por hardware con `xcap` (Tauri/Rust) y `desktopCapturer` (Electron).
- 🎨 **Herramientas de Anotación**: Rectángulos, elipses, flechas, pincel libre, mosaico, texto, deshacer/rehacer, cuentagotas.
- 📌 **Fijar al Escritorio**: Fije cualquier recorte como una ventana flotante arrastrable, redimensionable y siempre visible.
- 🖥️ **Multipantalla y HiDPI**: Mapeo automático de coordenadas multipantalla y escala de alta densidad de píxeles.

---

## Integración en Tauri (v2)

### 1. Instalar Dependencia

```bash
npm install @open-snapora/tauri
# o
pnpm add @open-snapora/tauri
```

### 2. Añadir Plugin de Rust

En `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

### 3. Configurar Permisos

En `src-tauri/capabilities/default.json`:

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

### 4. Registrar Plugin

En `src-tauri/src/main.rs`:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### 5. Tomar Captura

```typescript
import { capture, cancel, type ScreenshotResult } from '@open-snapora/tauri';

async function takeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      hideOwnerWindow: true, // Ocultar automáticamente la ventana anfitriona
    });

    if (result.status === 'completed') {
      console.log('Bytes de imagen:', result.data);
      console.log('Límites:', result.bounds);
      console.log('Acción de salida:', result.output);
    }
  } catch (error) {
    console.error('Error al capturar:', error);
  }
}
```

### 6. Compilar Release (Windows .exe)

```bash
tauri build
```

- **`.exe` Portátil Único**: Establezca `"bundle": { "active": false }` en `tauri.conf.json`.
- **Instalador (NSIS / MSI)**: Establezca `"bundle": { "active": true, "targets": ["nsis", "msi"] }` en `tauri.conf.json`.

---

## Integración en Electron

### 1. Instalar Dependencia

```bash
npm install @open-snapora/electron
# o
pnpm add @open-snapora/electron
```

### 2. Inicializar en el Proceso Principal

En su script de proceso principal:

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const { setupSnaporaMain } = require('@open-snapora/electron');

let snapora = null;

app.whenReady().then(() => {
  snapora = setupSnaporaMain({
    onComplete: (data) => {
      console.log('Captura completada:', data);
    },
    onCancel: (reason) => {
      console.log('Captura cancelada:', reason);
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

## Referencia de API

### `capture(options?: ScreenshotOptions): Promise<ScreenshotResult>`

| Opción | Tipo | Predeterminado | Descripción |
| :--- | :--- | :--- | :--- |
| `hideOwnerWindow` | `boolean` | `false` | Ocultar automáticamente la ventana actual antes de capturar |

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

## Proyectos de Ejemplo

Puede consultar ejemplos de implementación completos en el directorio `demos/`:
- `demos/tauri/`: Ejemplo en Tauri (v2)
- `demos/electron/`: Ejemplo en Electron

---

## Licencia

[MIT License](./LICENSE)
