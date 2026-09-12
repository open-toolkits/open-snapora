import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const currentDir = dirname(fileURLToPath(import.meta.url));
const overlayDist = resolve(currentDir, '../../overlay/dist');
const targetDir = resolve(currentDir, '../dist/overlay');

if (existsSync(overlayDist)) {
  if (existsSync(targetDir)) {
    rmSync(targetDir, { recursive: true, force: true });
  }
  mkdirSync(targetDir, { recursive: true });
  cpSync(overlayDist, targetDir, { recursive: true });

  // 注入完全自包含的 Tauri 原生桥接脚本
  const bridgeScript = `(function() {
  const internals = window.__TAURI_INTERNALS__;
  const invoke = internals ? internals.invoke : async () => {};

  function sendLog(tag, msg) {
    const text = typeof msg === 'object' ? JSON.stringify(msg) : String(msg);
    console.log('[' + tag + '] ' + text);
    invoke('plugin:snapora|log_message', { tag: tag, message: text }).catch(function() {});
  }

  window.addEventListener('error', function(err) {
    sendLog('Snapora:Overlay:UncaughtError', err.message + ' at ' + err.filename + ':' + err.lineno + ':' + err.colno);
  });

  window.addEventListener('unhandledrejection', function(err) {
    sendLog('Snapora:Overlay:UnhandledRejection', String(err.reason));
  });

  function listenEvent(eventName, callback) {
    if (!internals) return () => {};
    const handlerId = internals.transformCallback(function(e) {
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
      return listenEvent('plugin:snapora:initialize', async function(event) {
        const payload = event.payload;
        const frame = payload && payload.frames && payload.frames[0];
        if (frame && (frame.dataUrl === 'tauri-blob' || !frame.dataUrl || frame.dataUrl.length < 50)) {
          try {
            const dataUrl = await invoke('plugin:snapora|get_frame_image');
            frame.dataUrl = dataUrl;
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
      void invoke('plugin:snapora|overlay_ready').catch(function(e) {});
    },
    prepared(jobId) {
      void invoke('plugin:snapora|overlay_prepared', { jobId: jobId }).catch(function(e) {});
    },
    feedbackReady() {
      void invoke('plugin:snapora|feedback_ready').catch(function(e) {});
    },
    cancel(jobId) {
      void invoke('plugin:snapora|cancel', { jobId: jobId }).catch(function(e) {});
    },
    reportError(payload) {
      void invoke('plugin:snapora|report_error', { payload: payload }).catch(function(e) {});
    },
    async output(payload) {
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
      const b64 = payload.result && payload.result.data ? uint8ArrayToBase64(payload.result.data) : '';
      void invoke('plugin:snapora|confirm', {
        payload: Object.assign({}, payload, {
          result: payload.result ? Object.assign({}, payload.result, {
            dataBase64: b64,
            data: undefined
          }) : undefined
        })
      }).catch(function(e) {});
    }
  };
})();`;

  writeFileSync(resolve(targetDir, 'tauri-bridge.js'), bridgeScript, 'utf8');

  // 修改 index.html 引入桥接
  const htmlPath = resolve(targetDir, 'index.html');
  if (existsSync(htmlPath)) {
    let html = readFileSync(htmlPath, 'utf8');
    if (!html.includes('tauri-bridge.js')) {
      html = html.replace('<head>', '<head>\n    <script src="./tauri-bridge.js"></script>');
      writeFileSync(htmlPath, html, 'utf8');
    }
  }

  // 注入置顶贴图窗口专属桥接脚本
  const pinnedBridgeScript = `(function() {
  const internals = window.__TAURI_INTERNALS__;
  const invoke = internals ? internals.invoke : async () => {};

  function listenEvent(eventName, callback) {
    if (!internals) return () => {};
    const handlerId = internals.transformCallback(function(e) {
      callback(e);
    });
    invoke('plugin:event|listen', {
      event: eventName,
      target: { kind: 'Any' },
      handler: handlerId,
    }).catch(function(err) {});

    return function() {
      invoke('plugin:event|unlisten', { event: eventName, eventId: handlerId }).catch(function() {});
    };
  }

  function base64ToUint8Array(base64) {
    if (!base64) return new Uint8Array(0);
    const binary = atob(base64);
    const len = binary.length;
    const bytes = new Uint8Array(len);
    for (let i = 0; i < len; i++) {
      bytes[i] = binary.charCodeAt(i);
    }
    return bytes;
  }

  let cachedPayload = null;
  let initCallback = null;

  invoke('plugin:snapora|pinned_ready').then(function(res) {
    const bytes = base64ToUint8Array(res.dataBase64);
    cachedPayload = {
      data: bytes,
      mimeType: res.mimeType || 'image/png',
      locale: res.locale || 'zh-CN',
      menuLabels: res.menuLabels || {
        actions: '操作',
        copy: '复制 (Ctrl+C)',
        copied: '已复制到剪贴板',
        save: '保存为文件 (Ctrl+S)',
        close: '关闭 (Esc)'
      }
    };
    if (initCallback) {
      initCallback(cachedPayload);
    }
  }).catch(function(err) {});

  window.snaporaPinned = {
    onInitialize(listener) {
      initCallback = listener;
      if (cachedPayload) {
        listener(cachedPayload);
      }
      return function() {
        if (initCallback === listener) {
          initCallback = null;
        }
      };
    },
    onCopied(listener) {
      return listenEvent('plugin:snapora:pinned_copied', function() {
        listener();
      });
    },
    copy() {
      void invoke('plugin:snapora|pinned_copy').catch(function(e) {});
    },
    save() {
      void invoke('plugin:snapora|pinned_save').catch(function(e) {});
    },
    close() {
      void invoke('plugin:snapora|pinned_close').catch(function(e) {});
    },
    startDrag(point) {
      void invoke('plugin:snapora|pinned_start_drag', { point: point }).catch(function(e) {});
    },
    moveDrag(point) {
      void invoke('plugin:snapora|pinned_move_drag', { point: point }).catch(function(e) {});
    },
    endDrag() {
      void invoke('plugin:snapora|pinned_end_drag').catch(function(e) {});
    },
    resize(size) {
      void invoke('plugin:snapora|pinned_resize', { payload: size }).catch(function(e) {});
    }
  };
})();`;

  writeFileSync(resolve(targetDir, 'tauri-pinned-bridge.js'), pinnedBridgeScript, 'utf8');

  const pinnedHtmlPath = resolve(targetDir, 'pinned.html');
  if (existsSync(pinnedHtmlPath)) {
    let pinnedHtml = readFileSync(pinnedHtmlPath, 'utf8');
    if (!pinnedHtml.includes('tauri-pinned-bridge.js')) {
      pinnedHtml = pinnedHtml.replace('<head>', '<head>\n    <script src="./tauri-pinned-bridge.js"></script>');
      writeFileSync(pinnedHtmlPath, pinnedHtml, 'utf8');
    }
  }

  console.log('[open-snapora-tauri] Bundled overlay dist assets into @open-snapora/tauri/dist/overlay successfully.');
} else {
  console.warn('[open-snapora-tauri] Warning: packages/overlay/dist not found, build overlay first.');
}
