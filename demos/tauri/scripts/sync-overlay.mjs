import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { resolve, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { execSync } from 'node:child_process';

// 自动探测并清理之前残留占用 1420 端口的孤儿进程，防止 Vite 报 Port already in use
if (process.platform === 'win32') {
  try {
    const netstat = execSync('netstat -ano -p tcp', { encoding: 'utf8' });
    const lines = netstat.split('\n');
    for (const line of lines) {
      if (line.includes(':1420 ') && line.includes('LISTENING')) {
        const parts = line.trim().split(/\s+/);
        const pid = parts[parts.length - 1];
        if (pid && pid !== String(process.pid)) {
          console.log(`[demo-tauri] 发现残留孤儿进程 PID ${pid} 占用端口 1420，正在释放...`);
          try {
            execSync(`taskkill /F /PID ${pid}`, { stdio: 'ignore' });
          } catch {}
        }
      }
    }
  } catch {}
}

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
            const dataUrl = await invoke('plugin:snapora|get_frame_image');
            frame.dataUrl = dataUrl;
            sendLog('Snapora:Overlay', 'Fetched frame image in ' + Math.round(performance.now() - startFetch) + 'ms');
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

  // 修改 index.html 引入遮罩桥接
  const htmlPath = resolve(targetDir, 'index.html');
  let html = readFileSync(htmlPath, 'utf8');
  if (!html.includes('tauri-bridge.js')) {
    html = html.replace('<head>', '<head>\n    <script src="./tauri-bridge.js"></script>');
    writeFileSync(htmlPath, html, 'utf8');
  }

  // 注入针对置顶贴图窗口 (pinned.html) 的专属桥接脚本
  const pinnedBridgeScript = `(function() {
  const internals = window.__TAURI_INTERNALS__;
  const invoke = internals ? internals.invoke : async () => {};

  function sendLog(tag, msg) {
    const text = typeof msg === 'object' ? JSON.stringify(msg) : String(msg);
    console.log('[' + tag + '] ' + text);
    invoke('plugin:snapora|log_message', { tag: tag, message: text }).catch(function() {});
  }

  sendLog('Snapora:Pinned', 'Pinned tauri-pinned-bridge.js loaded and ready.');

  function listenEvent(eventName, callback) {
    if (!internals) return () => {};
    const handlerId = internals.transformCallback(function(e) {
      sendLog('Snapora:Pinned:Event', eventName);
      callback(e);
    });
    invoke('plugin:event|listen', {
      event: eventName,
      target: { kind: 'Any' },
      handler: handlerId,
    }).catch(function(err) {
      sendLog('Snapora:Pinned:ListenError', String(err));
    });

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

  // 页面加载即刻向 Rust 后台拉取贴图二进制及文案载荷
  invoke('plugin:snapora|pinned_ready').then(function(res) {
    sendLog('Snapora:Pinned', 'pinned_ready response received: ' + (res && res.dataBase64 ? res.dataBase64.length : 0));
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
  }).catch(function(err) {
    sendLog('Snapora:Pinned:Error', 'pinned_ready failed: ' + err);
  });

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
      sendLog('Snapora:Pinned', 'Pinned calling copy()');
      void invoke('plugin:snapora|pinned_copy').catch(function(e) {
        sendLog('Snapora:Pinned:Error', 'pinned_copy error: ' + e);
      });
    },
    save() {
      sendLog('Snapora:Pinned', 'Pinned calling save()');
      void invoke('plugin:snapora|pinned_save').catch(function(e) {
        sendLog('Snapora:Pinned:Error', 'pinned_save error: ' + e);
      });
    },
    close() {
      sendLog('Snapora:Pinned', 'Pinned calling close()');
      void invoke('plugin:snapora|pinned_close').catch(function(e) {
        sendLog('Snapora:Pinned:Error', 'pinned_close error: ' + e);
      });
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

  sendLog('Snapora:Bridge', 'Injected window.snaporaPinned bridge successfully.');
})();`;

  writeFileSync(resolve(targetDir, 'tauri-pinned-bridge.js'), pinnedBridgeScript, 'utf8');

  // 修改 pinned.html 引入贴图桥接
  const pinnedHtmlPath = resolve(targetDir, 'pinned.html');
  if (existsSync(pinnedHtmlPath)) {
    let pinnedHtml = readFileSync(pinnedHtmlPath, 'utf8');
    if (!pinnedHtml.includes('tauri-pinned-bridge.js')) {
      pinnedHtml = pinnedHtml.replace('<head>', '<head>\n    <script src="./tauri-pinned-bridge.js"></script>');
      writeFileSync(pinnedHtmlPath, pinnedHtml, 'utf8');
    }
  }

  console.log('[open-snapora-tauri] Synced overlay and pinned assets to public/overlay successfully.');
} else {
  console.warn('[open-snapora-tauri] Warning: packages/overlay/dist not found, please build packages/overlay first.');
}
