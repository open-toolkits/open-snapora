//! # tauri-plugin-snapora
//!
//! open-snapora 的 Tauri v2 原生插件，提供高性能跨平台屏幕截取、无边框透明窗口管理与剪贴板/文件系统输出。
//! 遵循解耦架构：
//! - Session 管生命周期
//! - ImageStore 管图片存储
//! - Core/Models 管业务模型
//! - Adapter/Commands 管框架协议
//! - Platform 管操作系统底层
//! - Application/CaptureService 管全链路业务编排

pub mod application;
pub mod capture;
pub mod commands;
pub mod error;
pub mod image;
pub mod logger;
pub mod models;
pub mod output;
pub mod pinned;
pub mod platform;
pub mod session;
pub mod window;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use error::Error;
pub use models::*;

/// 初始化 open-snapora Tauri 插件并注册全部分层服务与命令通道
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("snapora")
        .setup(|app, _api| {
            // 1. 初始化各领域核心管理器
            let session_mgr = session::SessionManager::new();
            let image_store = image::ImageStore::new();
            let output_mgr = output::OutputManager::new();
            let pinned_mgr = pinned::PinnedManager::new();

            // 2. 装配应用编排服务
            let capture_service = application::CaptureService::new(
                session_mgr.clone(),
                image_store.clone(),
                output_mgr.clone(),
            );

            // 3. 注册到全局 App 状态容器
            app.manage(session_mgr);
            app.manage(image_store);
            app.manage(output_mgr);
            app.manage(pinned_mgr);
            app.manage(capture_service);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::capture::capture,
            commands::capture::prewarm,
            commands::capture::get_frame_image,
            commands::session::cancel_active,
            commands::session::overlay_ready,
            commands::session::overlay_prepared,
            commands::session::feedback_ready,
            commands::session::cancel,
            commands::session::report_error,
            commands::session::confirm,
            commands::session::log_message,
            commands::session::get_log_path,
            commands::output::output,
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
    use crate::image::{ImageBuffer, ImageStore};
    use crate::models::{ImageMimeType, InitPayload, ScreenshotBounds, ScreenshotOptions, ScreenshotResult, SessionStatus};
    use crate::session::SessionManager;
    use crate::window::{SnapRegionCalculator, WindowProvider};
    use tokio::sync::oneshot;

    #[test]
    fn test_cancelled_serialization() {
        let res = ScreenshotResult::Cancelled;
        let json = serde_json::to_string(&res).unwrap();
        assert_eq!(json, r#"{"status":"cancelled"}"#);
    }

    #[test]
    fn test_image_store_isolation() {
        let store = ImageStore::new();
        let job1 = "job-100";
        let job2 = "job-200";

        let img1 = ImageBuffer::new(vec![1, 2, 3], ImageMimeType::Jpeg, 100, 100);
        let img2 = ImageBuffer::new(vec![4, 5, 6], ImageMimeType::Png, 200, 200);

        store.store_source(job1, img1);
        store.store_output(job2, img2);

        assert_eq!(store.get_source(job1).unwrap().bytes, vec![1, 2, 3]);
        assert!(store.get_source(job2).is_none());
        assert_eq!(store.get_output(job2).unwrap().bytes, vec![4, 5, 6]);

        store.remove(job1);
        assert!(store.get_source(job1).is_none());
    }

    #[test]
    fn test_session_job_id_strict_validation() {
        let mgr = SessionManager::new();
        let (tx1, mut rx1) = oneshot::channel();

        let payload = InitPayload {
            protocol_version: 2,
            job_id: "job-1".to_string(),
            options: ScreenshotOptions::default(),
            frames: vec![],
            window_snap_regions: None,
        };

        mgr.start_session("job-1".to_string(), payload.clone(), None, tx1);
        assert_eq!(mgr.get_status(Some("job-1")), Some(SessionStatus::Created));

        // 尝试用错误 job-2 执行状态更新，应该拒绝
        let err = mgr.update_status("job-2", SessionStatus::Editing);
        assert!(err.is_err());

        // 正确 job-1 状态更新
        let ok = mgr.update_status("job-1", SessionStatus::OverlayReady);
        assert!(ok.is_ok());
        assert_eq!(mgr.get_status(Some("job-1")), Some(SessionStatus::OverlayReady));

        // 开启新任务 job-2，job-1 应被强制取消
        let (tx2, _rx2) = oneshot::channel();
        mgr.start_session("job-2".to_string(), payload, None, tx2);

        // rx1 应该收到 Cancelled
        let res1 = rx1.try_recv().unwrap();
        match res1 {
            ScreenshotResult::Cancelled => {}
            _ => panic!("旧会话应收到 Cancelled"),
        }

        // 用 job-1 完成应该报错 StaleSession
        let stale_res = mgr.complete_session("job-1", ScreenshotResult::Cancelled);
        assert!(stale_res.is_err());
    }

    struct MockWindowProvider;
    impl WindowProvider for MockWindowProvider {
        fn windows(&self, _target_bounds: &ScreenshotBounds) -> Vec<ScreenshotBounds> {
            vec![
                // 有效相交窗口
                ScreenshotBounds { x: 50.0, y: 50.0, width: 200.0, height: 200.0 },
                // 太小的窗口应被过滤
                ScreenshotBounds { x: 10.0, y: 10.0, width: 20.0, height: 20.0 },
                // 完全在屏幕外的窗口
                ScreenshotBounds { x: 5000.0, y: 5000.0, width: 200.0, height: 200.0 },
            ]
        }
    }

    #[test]
    fn test_snap_region_calculator() {
        let provider = MockWindowProvider;
        let screen = ScreenshotBounds { x: 0.0, y: 0.0, width: 1920.0, height: 1080.0 };
        let snaps = SnapRegionCalculator::calculate(&provider, &screen);

        assert_eq!(snaps.len(), 1);
        assert_eq!(snaps[0].width, 200.0);
    }
}
