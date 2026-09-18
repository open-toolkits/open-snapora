# @open-snapora/tauri

> Suite multiplataforma de alto rendimiento para captura de pantalla, anotaciones y fijación en escritorio para **Tauri (v2)**.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/tauri.svg)](https://www.npmjs.com/package/@open-snapora/tauri)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/tauri.svg)](https://bundlephobia.com/package/@open-snapora/tauri)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/tauri.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://v2.tauri.app)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## Descripción General

`@open-snapora/tauri` es el adaptador de cliente oficial para integrar open-snapora en aplicaciones Tauri v2. Respaldado por el crate nativo de Rust [`tauri-plugin-snapora`](https://github.com/open-toolkits/open-snapora/tree/main/crates/tauri-plugin-snapora) y el motor `xcap`, ofrece captura de pantalla ultrarrápida en configuraciones multipantalla, una superposición interactiva con herramientas de anotación y ventanas flotantes fijadas en el escritorio (Pinned Window) con un consumo mínimo de recursos.

---

## Características Principales

- ⚡ **Motor Nativo Rust**: Captura acelerada por hardware con soporte completo para múltiples monitores y escalado HiDPI.
- 🚀 **Precalentamiento en Segundo Plano (`prewarm`)**: Precarga la ventana de superposición transparente y el entorno WebView2 al iniciar la aplicación, eliminando demoras y pantallas en blanco.
- 🪟 **Ajuste Inteligente a Ventanas (Window Snapping)**: Detecta automáticamente los bordes de las ventanas activas al pasar el cursor para seleccionarlas con un solo clic.
- 🎨 **Herramientas de Anotación Completas**:
  - **Formas**: Rectángulos, elipses, flechas y líneas rectas.
  - **Dibujo y Censura**: Pincel libre y filtro de mosaico / desenfoque para ocultar datos sensibles.
  - **Texto y Marcas**: Editor de texto enriquecido (fuentes, tamaños, contornos, relleno) y marcas de agua.
  - **Historial Completo**: Soporte ilimitado para Deshacer y Rehacer (`Undo` / `Redo`).
  - **Control de Estilo**: Selector de color (paleta preestablecida + HEX), control de grosor de trazo y opacidad.
- 📌 **Fijar en el Escritorio (Pin to Desktop)**: Fija cualquier captura como una ventana flotante sin bordes, siempre visible (Always-on-top), que se puede arrastrar, redimensionar, copiar o guardar.
- 📋 **Múltiples Opciones de Salida**:
  - Copia directa al portapapeles con notificación toast flotante (`showCopyFeedback`).
  - Diálogo nativo de guardado de archivo con retorno de ruta absoluta (`filePath`).
  - Creación de ventana flotante fijada en el escritorio.
- 🌐 **Internacionalización (i18n)**: Incluye español (`es-ES`), inglés (`en-US`), chino (`zh-CN`), japonés (`ja-JP`) y coreano (`ko-KR`), con capacidad de sobrescribir cualquier texto.
- 🎨 **Personalización Visual**: Modo claro y oscuro, colores de acento, fondos de máscara y estilos de barra de herramientas configurables.
- 🛡️ **Seguridad Nativa de Tauri v2**: Totalmente compatible con el modelo de permisos y Capabilities de Tauri v2.

---

## Instalación y Requisitos

### 1. Requisitos Previos

- **Frontend**: `@tauri-apps/api >= 2.0.0`
- **Backend Rust**: Tauri v2 (`tauri >= 2.0`), Rust `>= 1.77.2`

### 2. Instalar Paquete NPM

```bash
npm install @open-snapora/tauri
# o
pnpm add @open-snapora/tauri
# o
yarn add @open-snapora/tauri
```

### 3. Agregar Dependencia en Rust

En su archivo `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri = { version = "2.0", features = ["wry"] }
tauri-plugin-snapora = { git = "https://github.com/open-toolkits/open-snapora.git" }
```

---

## Inicio Rápido

### Paso 1: Configurar Capabilities

Agregue `snapora:default` en su archivo de capacidades (ej. `src-tauri/capabilities/default.json`):

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

### Paso 2: Registrar el Plugin en Rust

En `src-tauri/src/lib.rs` (o `main.rs`):

```rust
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("Error al ejecutar la aplicación Tauri");
}
```

### Paso 3: Invocar Captura desde el Frontend

```typescript
import { capture, prewarm, type ScreenshotResult } from '@open-snapora/tauri';

// Opcional: Precalentar la ventana al iniciar la app para respuesta instantánea
void prewarm().catch(console.error);

async function handleTakeScreenshot() {
  try {
    const result: ScreenshotResult = await capture({
      locale: 'es-ES',
      showCopyFeedback: true,
      defaultTool: 'select',
      tools: ['rectangle', 'ellipse', 'arrow', 'brush', 'text', 'mosaic', 'watermark'],
      theme: {
        mode: 'dark',
        accentColor: '#3b82f6',
      },
    });

    if (result.status === 'completed') {
      console.log('¡Captura completada!');
      console.log('Acción realizada:', result.output.action); // 'copy' | 'save' | 'pin'
      console.log('Coordenadas:', result.bounds);             // { x, y, width, height }
      console.log('Monitor:', result.displayId);
      console.log('Tamaño PNG:', result.data.byteLength);

      if (result.output.action === 'save') {
        console.log('Archivo guardado en:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('Captura cancelada por el usuario (Esc o botón Cancelar).');
    } else {
      console.error('Fallo en la captura:', result.code, result.message);
    }
  } catch (error) {
    console.error('Error inesperado durante la captura:', error);
  }
}
```

---

## Licencia

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
