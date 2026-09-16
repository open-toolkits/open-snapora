use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Runtime};
use tokio::sync::oneshot;

use crate::capture::{DefaultScreenCapture, ScreenCapture};
use crate::error::Error;
use crate::image::ImageStore;
use crate::models::{InitPayload, ScreenshotOptions, ScreenshotResult, SessionStatus};
use crate::output::OutputManager;
use crate::session::SessionManager;
use crate::window::{DefaultWindowProvider, SnapRegionCalculator, WindowProvider};
use crate::window::{hide_overlay_window, show_overlay_window};

/// 屏幕截图全生命周期应用服务（CaptureService）
/// 
/// 负责协调：
/// 1. 硬件屏幕采集 (ScreenCapture)
/// 2. 窗口边界枚举与智能吸附 (WindowProvider & SnapRegionCalculator)
/// 3. 二进制图片集中缓存 (ImageStore)
/// 4. 状态与会话生命周期严格流转 (SessionManager)
/// 5. 跨平台透明遮罩窗口交互 (Overlay Window)
/// 6. 多端输出与动作分发 (OutputManager)
#[derive(Clone)]
pub struct CaptureService {
    capture_provider: Arc<Box<dyn ScreenCapture>>,
    window_provider: Arc<Box<dyn WindowProvider>>,
    session_manager: SessionManager,
    image_store: ImageStore,
    output_manager: OutputManager,
}

impl Default for CaptureService {
    fn default() -> Self {
        Self::new(
            SessionManager::new(),
            ImageStore::new(),
            OutputManager::new(),
        )
    }
}

impl CaptureService {
    pub fn new(
        session_manager: SessionManager,
        image_store: ImageStore,
        output_manager: OutputManager,
    ) -> Self {
        Self {
            capture_provider: Arc::new(Box::new(DefaultScreenCapture::new())),
            window_provider: Arc::new(Box::new(DefaultWindowProvider::new())),
            session_manager,
            image_store,
            output_manager,
        }
    }

    pub fn with_providers(
        capture_provider: Box<dyn ScreenCapture>,
        window_provider: Box<dyn WindowProvider>,
        session_manager: SessionManager,
        image_store: ImageStore,
        output_manager: OutputManager,
    ) -> Self {
        Self {
            capture_provider: Arc::new(capture_provider),
            window_provider: Arc::new(window_provider),
            session_manager,
            image_store,
            output_manager,
        }
    }

    pub fn session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    pub fn image_store(&self) -> &ImageStore {
        &self.image_store
    }

    pub fn output_manager(&self) -> &OutputManager {
        &self.output_manager
    }

    /// 发起完整截图流程，返回异步 Promise 等待用户确认或取消
    pub async fn capture<R: Runtime>(
        &self,
        app: &AppHandle<R>,
        options: ScreenshotOptions,
    ) -> Result<ScreenshotResult, Error> {
        crate::logger::write_log("Snapora:CaptureService", &format!("开始执行 capture 流程: {options:?}"));

        // 1. 生成唯一任务 ID (jobId)
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        let job_id = format!("job-tauri-{timestamp}");

        // 2. 底层原生硬件截屏，获取图片帧及 ImageBuffer
        let (frame, image_buffer) = self.capture_provider.capture(&options)?;
        crate::logger::write_log(
            "Snapora:CaptureService",
            &format!("截屏成功: 物理分辨率 {}x{}", frame.pixel_size.width, frame.pixel_size.height),
        );

        // 3. 计算智能窗口吸附边界
        let window_snaps = SnapRegionCalculator::calculate(
            self.window_provider.as_ref().as_ref(),
            &frame.display.bounds,
        );
        crate::logger::write_log(
            "Snapora:CaptureService",
            &format!("窗口吸附计算完成，嗅探到 {} 个候选区域", window_snaps.len()),
        );

        // 4. 将高精度原始图片存入独立的 ImageStore（解耦 Session）
        let source_image_ref = image_buffer.to_ref(format!("{job_id}-source"));
        self.image_store.store_source(&job_id, image_buffer);

        let payload = InitPayload {
            protocol_version: 2,
            job_id: job_id.clone(),
            options: options.clone(),
            frames: vec![frame.clone()],
            window_snap_regions: if window_snaps.is_empty() {
                None
            } else {
                Some(window_snaps)
            },
        };

        // 5. 创建异步结果通道并注册到全局会话管理器，初始化状态为 Created
        let (tx, rx) = oneshot::channel::<ScreenshotResult>();
        self.session_manager.start_session(
            job_id.clone(),
            payload.clone(),
            Some(source_image_ref),
            tx,
        );
        let _ = self.session_manager.update_status(&job_id, SessionStatus::Capturing);

        // 6. 打开全屏透明置顶遮罩窗口
        crate::logger::write_log("Snapora:CaptureService", &format!("呼出 Overlay 遮罩窗口 (jobId: {job_id})..."));
        let _ = show_overlay_window(app, &frame, &payload)?;
        let _ = self.session_manager.update_status(&job_id, SessionStatus::OverlayReady);

        // 7. 异步阻塞等待用户在 Overlay 完成编辑或按 Esc 取消
        crate::logger::write_log("Snapora:CaptureService", "等待前端用户操作响应 (rx.await)...");
        let final_result = match rx.await {
            Ok(res) => {
                let status_str = match &res {
                    ScreenshotResult::Completed { data, bounds, output, .. } => {
                        format!("completed (bytes: {}, bounds: {bounds:?}, output: {output:?})", data.len())
                    }
                    ScreenshotResult::Cancelled => "cancelled".to_string(),
                    ScreenshotResult::Failed { code, message } => format!("failed: {code} - {message}"),
                };
                crate::logger::write_log("Snapora:CaptureService", &format!("收到会话结果: {status_str}"));
                res
            }
            Err(err) => {
                crate::logger::write_log(
                    "Snapora:CaptureService",
                    &format!("⚠️ rx.await 通道对端关闭 ({err:?})，降级为 Cancelled"),
                );
                ScreenshotResult::Cancelled
            }
        };

        // 8. 关闭遮罩窗口并清理 ImageStore
        let _ = hide_overlay_window(app);
        self.image_store.remove(&job_id);
        crate::logger::write_log("Snapora:CaptureService", &format!("任务 [{job_id}] 流程结束，遮罩已隐藏，资源已回收"));

        Ok(final_result)
    }
}
