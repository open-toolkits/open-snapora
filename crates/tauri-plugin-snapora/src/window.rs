use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use crate::error::Error;
use crate::models::{CapturedImageFrame, InitPayload};
use std::sync::atomic::{AtomicIsize, Ordering};

const OVERLAY_WINDOW_LABEL: &str = "snapora-overlay";

/// 记录调用截图前系统激活的前台窗口句柄，截图结束时精准归还焦点
static PREVIOUS_FOREGROUND_WINDOW: AtomicIsize = AtomicIsize::new(0);

#[cfg(target_os = "windows")]
type HWND = *mut std::ffi::c_void;
#[cfg(target_os = "windows")]
type BOOL = i32;

#[cfg(target_os = "windows")]
extern "system" {
    fn GetForegroundWindow() -> HWND;
    fn SetForegroundWindow(hWnd: HWND) -> BOOL;
    fn IsWindow(hWnd: HWND) -> BOOL;
}

/// 创建或复用覆盖整个目标屏幕的透明无边框 Overlay 窗口
pub fn show_overlay_window<R: Runtime>(
    app: &AppHandle<R>,
    frame: &CapturedImageFrame,
    payload: &InitPayload,
) -> Result<WebviewWindow<R>, Error> {
    // 记录呼出遮罩前的前台窗口
    #[cfg(target_os = "windows")]
    unsafe {
        let fg = GetForegroundWindow();
        PREVIOUS_FOREGROUND_WINDOW.store(fg as isize, Ordering::SeqCst);
    }
    // 1. 若窗口已存在，先获取已有窗口，否则新建透明遮罩窗口
    let window = if let Some(existing) = app.get_webview_window(OVERLAY_WINDOW_LABEL) {
        existing
    } else {
        WebviewWindowBuilder::new(
            app,
            OVERLAY_WINDOW_LABEL,
            WebviewUrl::App("overlay/index.html".into()),
        )
        .title("open-snapora Overlay")
        .transparent(true)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .shadow(false)
        .resizable(false)
        .build()
        .map_err(|err| Error::WindowError(format!("创建透明全屏窗口失败: {err}")))?
    };

    // 2. 将窗口精准移动至目标屏幕物理坐标并覆盖整个显示器
    let pos = PhysicalPosition::new(frame.display.bounds.x as i32, frame.display.bounds.y as i32);
    let size = PhysicalSize::new(
        frame.pixel_size.width,
        frame.pixel_size.height,
    );

    let _ = window.set_position(pos);
    let _ = window.set_size(size);
    let _ = window.show();
    let _ = window.set_focus();

    // 3. 同时向 Webview 广播初始化数据载荷（供已存在并监听中的窗口使用）
    let _ = app.emit("plugin:snapora:initialize", payload);

    Ok(window)
}

/// 预热透明遮罩窗口，在插件注册时即刻在后台创建并隐藏，提前完成 WebView2 运行时及静态资源加载
pub fn prewarm_overlay_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    if app.get_webview_window(OVERLAY_WINDOW_LABEL).is_none() {
        crate::logger::write_log("Snapora:Rust", "Pre-warming overlay window in background...");
        let window = WebviewWindowBuilder::new(
            app,
            OVERLAY_WINDOW_LABEL,
            WebviewUrl::App("overlay/index.html".into()),
        )
        .title("open-snapora Overlay")
        .transparent(true)
        .decorations(false)
        .always_on_top(true)
        .skip_taskbar(true)
        .shadow(false)
        .resizable(false)
        .visible(false)
        .build()
        .map_err(|err| Error::WindowError(format!("预热透明全屏窗口失败: {err}")))?;

        let _ = window.hide();
        crate::logger::write_log("Snapora:Rust", "Overlay window pre-warmed successfully.");
    }
    Ok(())
}

/// 隐藏并清理 Overlay 窗口，并将系统前台焦点归还给截图前的应用
pub fn hide_overlay_window<R: Runtime>(app: &AppHandle<R>) -> Result<(), Error> {
    if let Some(window) = app.get_webview_window(OVERLAY_WINDOW_LABEL) {
        let _ = window.hide();
    }

    #[cfg(target_os = "windows")]
    unsafe {
        let prev = PREVIOUS_FOREGROUND_WINDOW.swap(0, Ordering::SeqCst);
        if prev != 0 {
            let hwnd = prev as HWND;
            if IsWindow(hwnd) != 0 {
                let _ = SetForegroundWindow(hwnd);
            }
        }
    }
    Ok(())
}
