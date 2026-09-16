use tauri::{command, AppHandle, Manager, Runtime};
use crate::application::CaptureService;
use crate::error::Error;
use crate::models::{ScreenshotOptions, ScreenshotResult};
use crate::window;

/// 发起截图会话（IPC 薄适配器，直接转发给 CaptureService 执行）
#[command]
pub async fn capture<R: Runtime>(
    app: AppHandle<R>,
    options: ScreenshotOptions,
) -> Result<ScreenshotResult, Error> {
    let service = app.state::<CaptureService>();
    service.capture(&app, options).await
}

/// 在后台静默预热透明截图遮罩窗口（提前加载 WebView2 运行时与 UI 资源，避免冷启动延迟）
#[command]
pub async fn prewarm<R: Runtime>(app: AppHandle<R>) -> Result<(), Error> {
    window::prewarm_overlay_window(&app)
}

/// 前端拉取全屏截图原图 Base64 DataURL（作为兜底支持）
#[command]
pub async fn get_frame_image<R: Runtime>(
    app: AppHandle<R>,
) -> Result<String, Error> {
    let service = app.state::<CaptureService>();
    let session_mgr = service.session_manager();
    let image_store = service.image_store();

    if let Some(payload) = session_mgr.get_init_payload(None) {
        if let Some(frame) = payload.frames.first() {
            if frame.data_url.starts_with("data:image/") {
                return Ok(frame.data_url.clone());
            }
        }
        if let Some(source_img) = image_store.get_source(&payload.job_id) {
            let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &source_img.bytes);
            return Ok(format!("data:{};base64,{b64}", source_img.mime_type.as_str()));
        }
    }

    Err(Error::CaptureFailed("未找到当前截屏图像数据".to_string()))
}
