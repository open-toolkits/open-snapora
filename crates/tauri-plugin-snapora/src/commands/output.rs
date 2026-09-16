use tauri::{command, AppHandle, Manager, Runtime};
use crate::application::CaptureService;
use crate::error::Error;
use crate::image::{ImageBuffer, ImageMimeType};
use crate::models::{ScreenshotBounds, ScreenshotOutputResponse};
use crate::output::OutputContext;
use super::session::extract_image_bytes;

/// 执行保存、复制、贴图等原生动作并返回响应（薄适配器，委派给 OutputManager 或 Pinned 模块）
#[command]
pub async fn output<R: Runtime>(
    app: AppHandle<R>,
    payload: serde_json::Value,
) -> Result<ScreenshotOutputResponse, Error> {
    let action = payload.get("action").and_then(|v| v.as_str()).unwrap_or("copy");
    let job_id = payload.get("jobId").and_then(|v| v.as_str()).map(|s| s.to_string());
    crate::logger::write_log("Snapora:Commands:Output", &format!("output() 触发: action = {action}, jobId = {job_id:?}"));

    let service = app.state::<CaptureService>();
    let session_mgr = service.session_manager();
    let image_store = service.image_store();
    let output_mgr = service.output_manager();

    let active_job_id = job_id.clone().or_else(|| session_mgr.current_job_id());

    // 1. 提取图像数据并存入 ImageStore
    let data_bytes = extract_image_bytes(payload.get("result"));
    if !data_bytes.is_empty() {
        let buffer = ImageBuffer::new(
            data_bytes.clone(),
            ImageMimeType::Png,
            0,
            0,
        );
        if let Some(ref jid) = active_job_id {
            image_store.store_output(jid, buffer);
        }
    }

    // 2. 构造输出上下文与选区边界
    let custom_path = payload.get("filePath").and_then(|v| v.as_str()).map(|s| s.to_string());
    let init_payload = session_mgr.get_init_payload(active_job_id.as_deref());
    let locale = init_payload.as_ref().and_then(|p| p.options.locale.clone());

    let bounds = payload
        .get("result")
        .and_then(|r| r.get("bounds"))
        .and_then(|b| {
            Some(ScreenshotBounds {
                x: b.get("x")?.as_f64()?,
                y: b.get("y")?.as_f64()?,
                width: b.get("width")?.as_f64()?,
                height: b.get("height")?.as_f64()?,
            })
        })
        .unwrap_or(ScreenshotBounds { x: 0.0, y: 0.0, width: 200.0, height: 200.0 });

    // 3. 若为 pin 动作，调度 Pinned 模块创建独立置顶贴图小窗口
    if action == "pin" {
        crate::pinned::create_pinned_window(&app, &data_bytes, &bounds, locale.as_deref())?;
        crate::logger::write_log("Snapora:Commands:Output", "output() -> Pinned window 创建成功");
        return Ok(ScreenshotOutputResponse::Completed {
            action: "pin".to_string(),
            file_path: None,
        });
    }

    let context = OutputContext {
        job_id: active_job_id,
        action: action.to_string(),
        file_path: custom_path,
        bounds,
        locale,
    };

    let image_buf = ImageBuffer::new(
        data_bytes,
        ImageMimeType::Png,
        0,
        0,
    );

    // 4. 统一通过 OutputManager 执行并返回
    output_mgr.execute(&image_buf, &context)
}
