# @open-snapora/electron

> Administrador de captura de pantalla multipantalla, anotación en lienzo y fijación en el escritorio para **Electron**.

[English](README.md) | [简体中文](README.zh-CN.md) | [日本語](README.ja.md) | [Español](README.es.md) | [한국어](README.ko.md)

[![npm version](https://img.shields.io/npm/v/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![npm downloads](https://img.shields.io/npm/dm/@open-snapora/electron.svg)](https://www.npmjs.com/package/@open-snapora/electron)
[![bundle size](https://img.shields.io/bundlephobia/minzip/@open-snapora/electron.svg)](https://bundlephobia.com/package/@open-snapora/electron)
[![TypeScript](https://img.shields.io/badge/TypeScript-%E2%9C%93-3178C6.svg)](https://www.typescriptlang.org/)
[![license](https://img.shields.io/npm/l/@open-snapora/electron.svg)](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
[![Electron >= 30](https://img.shields.io/badge/Electron-%3E%3D30.0.0-47848F.svg)](https://www.electronjs.org/)
[![CI](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml/badge.svg)](https://github.com/open-toolkits/open-snapora/actions/workflows/ci.yml)

---

## Descripción General

`@open-snapora/electron` proporciona una solución robusta y lista para producción de captura de pantalla, anotaciones interactivas y ventanas flotantes fijadas en el escritorio para aplicaciones de escritorio desarrolladas con Electron. Maneja la transformación de coordenadas entre múltiples monitores, el escalado HiDPI y la reutilización de ventanas de superposición transparentes sin parpadeos ni pantallas en blanco.

---

## Características Principales

- 🖥️ **Captura Multipantalla Nativa**: Combina con precisión los espacios de coordenadas físicas y lógicas en múltiples pantallas, admitiendo resoluciones Retina y HiDPI.
- ⚡ **Reutilización de Ventana sin Pantallazos en Blanco**: Almacena en caché y reutiliza ventanas de superposición transparentes en segundo plano, logrando una apertura inmediata.
- 🪟 **Ajuste Inteligente a Ventanas (Window Snapping)**: Detecta y resalta automáticamente los bordes de las ventanas activas al pasar el cursor; permite inyectar coordenadas de ventanas nativas del sistema operativo a través de `getWindowSnapRegions`.
- 📌 **Ventanas Flotantes Fijadas en el Escritorio (Pin to Desktop)**: Fija capturas en el escritorio en ventanas independientes sin bordes (Always-on-top), con soporte para arrastrar, redimensionar, copiar, guardar y cierre sincronizado con el ciclo de vida de la aplicación.
- 🎨 **Herramientas de Anotación Completas**:
  - **Geometría**: Rectángulos, elipses, flechas y líneas.
  - **Pincel y Censura**: Dibujo libre y filtro de mosaico / desenfoque con intensidad regulable.
  - **Texto y Marcas de Agua**: Entrada de texto enriquecido (tamaño, color, borde, relleno) y marcas de agua.
  - **Historial Completo**: Soporte ilimitado de Deshacer y Rehacer (`Undo` / `Redo`).
  - **Estilos**: Selector de color (paleta predefinida + HEX), grosor de trazo y opacidad.
- 🛡️ **Arquitectura de Seguridad Empresarial**:
  - Basado estrictamente en **Context Isolation** (`contextIsolation: true`), sin depender del módulo inseguro `remote`.
  - Validación de origen con `validateSender` para bloquear iframes maliciosos y llamadas IPC no autorizadas.
- 🚥 **Control de Concurrencia (`busyPolicy`)**: Modos `'queue'` (cola FIFO para solicitudes repetidas) y `'reject'` (rechazo inmediato).
- 🔌 **Canal de Salida Personalizable (`outputAdapter`)**: Permite redirigir la salida directamente a servicios en la nube (AWS S3, OSS, APIs internas) en lugar del disco local.
- 📊 **Telemetría y Diagnóstico (`onDiagnostic`)**: Registro de tiempos exactos en microsegundos para cada etapa (`capture`, `overlay`, `edit`, `output`).
- 🌐 **Internacionalización (i18n)**: Compatible de forma nativa con español (`es-ES`), inglés (`en-US`), chino (`zh-CN`), japonés (`ja-JP`) y coreano (`ko-KR`).

---

## Instalación

```bash
npm install @open-snapora/electron
# o
pnpm add @open-snapora/electron
# o
yarn add @open-snapora/electron
```

> **Requisito**: `electron >= 30.0.0`.

---

## Inicio Rápido

### 1. Proceso Principal (`main.js` / `main.ts`)

```javascript
const { app, BrowserWindow, globalShortcut } = require('electron');
const path = require('path');
const { setupElectronSnapora, resolveHostPreloadPath } = require('@open-snapora/electron');

// 1. Inicializar el controlador Snapora
const snapora = setupElectronSnapora({
  busyPolicy: 'queue',
});

let mainWindow = null;

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 1024,
    height: 768,
    webPreferences: {
      preload: resolveHostPreloadPath(), // Inyecta window.electronSnapora de forma segura
      contextIsolation: true,
      nodeIntegration: false,
    },
  });

  mainWindow.loadFile('index.html');
}

app.whenReady().then(() => {
  createWindow();

  // 2. Registrar atajo global
  const shortcut = process.platform === 'darwin' ? 'Command+Alt+A' : 'Ctrl+Alt+A';
  globalShortcut.register(shortcut, () => {
    snapora.manager.capture({
      locale: 'es-ES',
      showCopyFeedback: true,
    });
  });
});

app.on('will-quit', () => {
  globalShortcut.unregisterAll();
  snapora.unregister();
});
```

### 2. Proceso de Renderizado (`renderer.js`)

```typescript
async function handleTakeScreenshot() {
  try {
    const result = await window.electronSnapora.capture({
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
      console.log('Acción realizada:', result.output.action);
      console.log('Dimensiones:', result.bounds);

      if (result.output.action === 'save') {
        console.log('Archivo guardado en:', result.output.filePath);
      }
    } else if (result.status === 'cancelled') {
      console.log('Captura cancelada.');
    } else {
      console.error('Error de captura:', result.code, result.message);
    }
  } catch (error) {
    console.error('Error inesperado:', error);
  }
}
```

---

## Ejemplo Completo

Consulte la aplicación de demostración completa en el directorio [`demos/electron`](https://github.com/open-toolkits/open-snapora/tree/main/demos/electron).

---

## Licencia

MIT © [open-snapora authors](https://github.com/open-toolkits/open-snapora/blob/main/LICENSE)
