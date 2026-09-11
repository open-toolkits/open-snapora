import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const currentDir = dirname(fileURLToPath(import.meta.url));
const overlayDist = resolve(currentDir, '../../../packages/overlay/dist');
const targetDir = resolve(currentDir, '../public/overlay');

if (existsSync(overlayDist)) {
  if (existsSync(targetDir)) {
    rmSync(targetDir, { recursive: true, force: true });
  }
  mkdirSync(targetDir, { recursive: true });
  cpSync(overlayDist, targetDir, { recursive: true });

  // 注入完全自包含的 Tauri 原生桥接层，无需外部模块依赖
  const bridgeScript = `(function() {
  const internals = window.__TAURI_INTERNALS__;
  const invoke = internals ? internals.invoke : async () => {};

  function sendLog(tag, msg) {
    const text = typeof msg === 'object' ? JSON.stringify(msg) : String(msg);
    console.log('[' + tag + '] ' + text);
    invoke('plugin:snapora|log_message', { tag: tag, message: text }).catch(function() {});
  }

  sendLog('Snapora:Overlay', 'Overlay tauri-bridge.js loaded and ready.');

  window.addEventListener('error', function(err) {
    const errText = err.message + ' at ' + err.filename + ':' + err.lineno + ':' + err.colno;
    sendLog('Snapora:Overlay:UncaughtError', errText);
  });

  window.addEventListener('unhandledrejection', function(err) {
    sendLog('Snapora:Overlay:UnhandledRejection', String(err.reason));
  });

  function listenEvent(eventName, callback) {
    if (!internals) return () => {};
    const handlerId = internals.transformCallback(function(e) {
      sendLog('Snapora:Overlay:Event', eventName);
      callback(e);
    });
    invoke('plugin:event|listen', {
      event: eventName,
      target: { kind: 'Any' },
      handler: handlerId,
    }).catch(function(err) {
      sendLog('Snapora:Overlay:ListenError', String(err));
    });

    return function() {
      invoke('plugin:event|unlisten', { event: eventName, eventId: handlerId }).catch(function() {});
    };
  }

  function uint8ArrayToBase64(bytes) {
    if (!bytes || bytes.length === 0) return '';
    let binary = '';
    const len = bytes.byteLength || bytes.length;
    const chunkSize = 0x8000;
    for (let i = 0; i < len; i += chunkSize) {
      binary += String.fromCharCode.apply(null, bytes.subarray ? bytes.subarray(i, Math.min(i + chunkSize, len)) : bytes.slice(i, Math.min(i + chunkSize, len)));
    }
    return btoa(binary);
  }

  window.snaporaOverlay = {
    onInitialize(listener) {
      sendLog('Snapora:Overlay', 'onInitialize listener registered');
      return listenEvent('plugin:snapora:initialize', async function(event) {
        sendLog('Snapora:Overlay', 'Received initialize payload, jobId: ' + (event.payload && event.payload.jobId));
        const payload = event.payload;
        const frame = payload && payload.frames && payload.frames[0];
        if (frame && (frame.dataUrl === 'tauri-blob' || !frame.dataUrl || frame.dataUrl.length < 50)) {
          try {
            const startFetch = performance.now();
            const bytes = await invoke('plugin:snapora|get_frame_image');
            const blob = new Blob([bytes], { type: 'image/png' });
            frame.dataUrl = URL.createObjectURL(blob);
            sendLog('Snapora:Overlay', 'Fetched frame binary IPC via ArrayBuffer in ' + Math.round(performance.now() - startFetch) + 'ms');
          } catch(err) {
            sendLog('Snapora:Overlay:Error', 'get_frame_image failed: ' + err);
          }
        }
        listener(payload);
      });
    },
    onFeedback(listener) {
      return listenEvent('plugin:snapora:feedback', function(event) {
        listener(event.payload);
      });
    },
    ready() {
      sendLog('Snapora:Overlay', 'Overlay calling ready()...');
      void invoke('plugin:snapora|overlay_ready').catch(function(e) {
        sendLog('Snapora:Overlay:Error', 'overlay_ready failed: ' + e);
      });
    },
    prepared(jobId) {
      sendLog('Snapora:Overlay', 'Overlay image prepared, jobId: ' + jobId);
      void invoke('plugin:snapora|overlay_prepared', { jobId: jobId }).catch(function(e) {
        sendLog('Snapora:Overlay:Error', 'overlay_prepared failed: ' + e);
      });
    },
    feedbackReady() {
      void invoke('plugin:snapora|feedback_ready').catch(function(e) {});
    },
    cancel(jobId) {
      sendLog('Snapora:Overlay:Cancel', 'Overlay calling cancel()! jobId: ' + jobId);
      void invoke('plugin:snapora|cancel', { jobId: jobId }).catch(function(e) {
        sendLog('Snapora:Overlay:Error', 'cancel failed: ' + e);
      });
    },
    reportError(payload) {
      sendLog('Snapora:Overlay:ReportError', 'Overlay calling reportError(): ' + JSON.stringify(payload));
      void invoke('plugin:snapora|report_error', { payload: payload }).catch(function(e) {
        sendLog('Snapora:Overlay:Error', 'report_error failed: ' + e);
      });
    },
    async output(payload) {
      sendLog('Snapora:Overlay', 'Overlay calling output(): ' + (payload && payload.action));
      const b64 = payload.result && payload.result.data ? uint8ArrayToBase64(payload.result.data) : '';
      return await invoke('plugin:snapora|output', {
        payload: Object.assign({}, payload, {
          result: payload.result ? Object.assign({}, payload.result, {
            dataBase64: b64,
            data: undefined
          }) : undefined
        })
      });
    },
    confirm(payload) {
      sendLog('Snapora:Overlay', 'Overlay calling confirm(), status: ' + (payload && payload.result && payload.result.status));
      const b64 = payload.result && payload.result.data ? uint8ArrayToBase64(payload.result.data) : '';
      void invoke('plugin:snapora|confirm', {
        payload: Object.assign({}, payload, {
          result: payload.result ? Object.assign({}, payload.result, {
            dataBase64: b64,
            data: undefined
          }) : undefined
        })
      }).catch(function(e) {
        sendLog('Snapora:Overlay:Error', 'confirm invoke failed: ' + e);
      });
    }
  };

  window.addEventListener('beforeunload', function(e) {
    sendLog('Snapora:Overlay:Lifecycle', 'Overlay window beforeunload triggered');
  });
  window.addEventListener('unload', function(e) {
    sendLog('Snapora:Overlay:Lifecycle', 'Overlay window unload triggered');
  });

  sendLog('Snapora:Bridge', 'Injected window.snaporaOverlay bridge successfully.');
})();`;

  writeFileSync(resolve(targetDir, 'tauri-bridge.js'), bridgeScript, 'utf8');

  // 修改 index.html 引入桥接
  const htmlPath = resolve(targetDir, 'index.html');
  let html = readFileSync(htmlPath, 'utf8');
  if (!html.includes('tauri-bridge.js')) {
    html = html.replace('<head>', '<head>\n    <script src=\"./tauri-bridge.js\"></script>');
    writeFileSync(htmlPath, html, 'utf8');
  }

  console.log('[open-snapora-tauri] Synced overlay assets to public/overlay successfully.');
} else {
  console.warn('[open-snapora-tauri] Warning: packages/overlay/dist not found, please build packages/overlay first.');
}
