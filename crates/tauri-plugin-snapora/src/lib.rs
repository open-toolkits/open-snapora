//! # tauri-plugin-snapora
//!
//! open-snapora 的 Tauri v2 原生插件，提供高性能跨平台屏幕截取、无边框透明窗口管理与剪贴板/文件系统输出。

pub mod capture;
pub mod commands;
pub mod error;
pub mod logger;
pub mod models;
pub mod output;
pub mod pinned;
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
            app.manage(pinned::PinnedManager::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::capture,
            commands::prewarm,
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
            pinned::pinned_ready,
            pinned::pinned_copy,
            pinned::pinned_save,
            pinned::pinned_close,
            pinned::pinned_start_drag,
            pinned::pinned_move_drag,
            pinned::pinned_end_drag,
            pinned::pinned_resize,
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
