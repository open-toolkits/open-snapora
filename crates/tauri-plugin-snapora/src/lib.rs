//! # tauri-plugin-snapora
//!
//! open-snapora 的 Tauri v2 原生插件，提供高性能跨平台屏幕截取、无边框透明窗口管理与剪贴板/文件系统输出。

pub mod capture;
pub mod commands;
pub mod error;
pub mod logger;
pub mod models;
pub mod output;
pub mod session;
pub mod window;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use error::Error;
pub use models::*;

/// 初始化 open-snapora Tauri 插件并注册所有命令处理通道
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("snapora")
        .setup(|app, _api| {
            app.manage(session::SessionManager::new());
            // 预热透明置顶遮罩窗口，提前初始化 WebView2 运行时，彻底消除首次截图 2 秒冷启动延迟
            let _ = window::prewarm_overlay_window(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::capture,
            commands::cancel_active,
            commands::overlay_ready,
            commands::overlay_prepared,
            commands::feedback_ready,
            commands::cancel,
            commands::report_error,
            commands::output,
            commands::confirm,
            commands::log_message,
            commands::get_log_path,
            commands::get_frame_image,
        ])
        .build()
}

#[cfg(test)]
mod tests {
    use crate::models::ScreenshotResult;

    #[test]
    fn test_cancelled_serialization() {
        let res = ScreenshotResult::Cancelled;
        let json = serde_json::to_string(&res).unwrap();
        assert_eq!(json, r#"{"status":"cancelled"}"#);
    }
}
