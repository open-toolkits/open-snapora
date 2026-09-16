use tauri::{command, AppHandle, Emitter, Manager, Runtime};
use crate::application::CaptureService;
use crate::error::Error;
use crate::models::{OutputMetadata, ScreenshotBounds, ScreenshotResult, SessionStatus};
use crate::window;

/// 从前端载荷中极速提取图片数据（优先 Base64 极速解码，兜底兼容 Array）
pub(crate) fn extract_image_bytes(res_obj: Option<&serde_json::Value>) -> Vec<u8> {
    let r = match res_obj {
        Some(val) => val,
        None => return Vec::new(),
    };

    // 1. 优先尝试解析高效 Base64 字符串（耗时仅 ~3ms）
    if let Some(b64) = r.get("dataBase64").and_then(|v| v.as_str()) {
        if let Ok(bytes) = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, b64) {
            return bytes;
        }
    }

    // 2. 兜底兼容传统数组
    r.get("data")
        .and_then(|d| d.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_u64().map(|n| n as u8))
                .collect::<Vec<u8>>()
        })
        .unwrap_or_default()
}

/// 取消当前进行中的截图会话
#[command]
pub async fn cancel_active<R: Runtime>(app: AppHandle<R>) -> Result<bool, Error> {
    crate::logger::write_log("Snapora:Commands:Session", "cancel_active() called");
    let service = app.state::<CaptureService>();
    let _ = service.session_manager().cancel_session(None);
    let _ = window::hide_overlay_window(&app);
    Ok(true)
}

/// Overlay 前端汇报已就绪，补发初始化帧数据以防丢包
#[command]
pub async fn overlay_ready<R: Runtime>(app: AppHandle<R>) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Commands:Session", "overlay_ready() called from frontend Webview");
    let service = app.state::<CaptureService>();
    if let Some(payload) = service.session_manager().get_init_payload(None) {
        crate::logger::write_log(
            "Snapora:Commands:Session",
            &format!("Re-emitting plugin:snapora:initialize with job_id: {}", payload.job_id),
        );
        let _ = service.session_manager().update_status(&payload.job_id, SessionStatus::OverlayReady);
        let _ = app.emit("plugin:snapora:initialize", payload);
    } else {
        crate::logger::write_log("Snapora:Commands:Session", "Warning: overlay_ready called but no active session found!");
    }
    Ok(())
}

/// Overlay 前端汇报图像已渲染完成
#[command]
pub async fn overlay_prepared<R: Runtime>(
    app: AppHandle<R>,
    job_id: Option<String>,
) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Commands:Session", &format!("overlay_prepared() called from frontend: {job_id:?}"));
    let service = app.state::<CaptureService>();
    if let Some(ref jid) = job_id {
        let _ = service.session_manager().update_status(jid, SessionStatus::Editing);
    }
    Ok(())
}

/// 反馈提示图层已就绪
#[command]
pub async fn feedback_ready<R: Runtime>(_app: AppHandle<R>) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Commands:Session", "feedback_ready() called");
    Ok(())
}

/// 用户在 Overlay 界面点击取消或按下 Esc（携带并校验 jobId）
#[command]
pub async fn cancel<R: Runtime>(
    app: AppHandle<R>,
    job_id: Option<String>,
) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Commands:Session", &format!("cancel() called from frontend! job_id: {job_id:?}"));
    let service = app.state::<CaptureService>();
    let _ = service.session_manager().cancel_session(job_id.as_deref());
    let _ = window::hide_overlay_window(&app);
    Ok(())
}

/// Overlay 上报错误（严格校验 jobId）
#[command]
pub async fn report_error<R: Runtime>(
    app: AppHandle<R>,
    payload: serde_json::Value,
) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Commands:Session", &format!("report_error() called from frontend! payload: {payload:?}"));
    let job_id = payload.get("jobId").and_then(|v| v.as_str());
    let code = payload.get("code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
    let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or("Error");

    let service = app.state::<CaptureService>();
    let active_job_id = job_id
        .map(|s| s.to_string())
        .or_else(|| service.session_manager().current_job_id());

    if let Some(jid) = active_job_id {
        let _ = service.session_manager().complete_session(&jid, ScreenshotResult::Failed {
            code: code.to_string(),
            message: message.to_string(),
        });
    }

    let _ = window::hide_overlay_window(&app);
    Ok(())
}

/// 确认截图最终结果并唤醒会话等待者（严格校验 jobId，消除竞态与状态错乱）
#[command]
pub async fn confirm<R: Runtime>(
    app: AppHandle<R>,
    payload: serde_json::Value,
) -> Result<(), Error> {
    // 收到 confirm 立即隐藏全屏遮罩并归还系统焦点，彻底消除鼠标消失与卡顿
    let _ = window::hide_overlay_window(&app);

    crate::logger::write_log(
        "Snapora:Commands:Session",
        &format!("confirm() called with payload status: {:?}", payload.get("result").and_then(|r| r.get("status"))),
    );

    let service = app.state::<CaptureService>();
    let session_mgr = service.session_manager();
    let image_store = service.image_store();

    let job_id = payload
        .get("jobId")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| session_mgr.current_job_id());

    let jid = match job_id {
        Some(id) => id,
        None => {
            crate::logger::write_log("Snapora:Commands:Session", "confirm() 警告: 未找到对应的有效任务 ID");
            return Ok(());
        }
    };

    let res = payload.get("result");
    let status = res.and_then(|r| r.get("status")).and_then(|s| s.as_str()).unwrap_or("cancelled");

    if status == "completed" {
        if let Some(r) = res {
            let mut data_bytes = extract_image_bytes(Some(r));
            // 若 confirm 阶段未传图片或为空，直接复用 ImageStore 缓存的 output 字节
            if data_bytes.is_empty() {
                if let Some(cached) = image_store.get_output(&jid).or_else(|| image_store.get_latest_output()) {
                    data_bytes = cached.bytes;
                }
            }

            let action_str = r
                .get("output")
                .and_then(|o| o.get("action"))
                .and_then(|a| a.as_str())
                .unwrap_or("copy");

            let bounds = r.get("bounds").and_then(|b| {
                Some(ScreenshotBounds {
                    x: b.get("x")?.as_f64()?,
                    y: b.get("y")?.as_f64()?,
                    width: b.get("width")?.as_f64()?,
                    height: b.get("height")?.as_f64()?,
                })
            }).unwrap_or(ScreenshotBounds { x: 0.0, y: 0.0, width: 0.0, height: 0.0 });

            let display_id = r.get("displayId").and_then(|d| d.as_str()).unwrap_or("primary").to_string();

            let output_meta = if action_str == "save" {
                OutputMetadata::Save { file_path: "saved".to_string() }
            } else if action_str == "pin" {
                OutputMetadata::Pin
            } else {
                OutputMetadata::Copy
            };

            let _ = session_mgr.complete_session(&jid, ScreenshotResult::Completed {
                data: data_bytes,
                mime_type: "image/png".to_string(),
                bounds,
                display_id,
                output: output_meta,
            });
        } else {
            let _ = session_mgr.cancel_session(Some(&jid));
        }
    } else {
        let _ = session_mgr.cancel_session(Some(&jid));
    }

    let _ = window::hide_overlay_window(&app);
    Ok(())
}

/// 接收来自前端渲染进程或 Overlay 遮罩层的日志并写入磁盘 app.log
#[command]
pub async fn log_message<R: Runtime>(
    _app: AppHandle<R>,
    tag: String,
    message: String,
) -> Result<(), Error> {
    crate::logger::write_log(&tag, &message);
    Ok(())
}

/// 获取系统目录下的日志文件绝对路径
#[command]
pub async fn get_log_path<R: Runtime>(_app: AppHandle<R>) -> Result<String, Error> {
    Ok(crate::logger::get_log_file_path().to_string_lossy().to_string())
}
