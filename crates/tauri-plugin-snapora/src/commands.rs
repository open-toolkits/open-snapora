use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{command, AppHandle, Emitter, Manager, Runtime};
use tokio::sync::oneshot;
use crate::capture;
use crate::error::Error;
use crate::models::{InitPayload, OutputMetadata, ScreenshotBounds, ScreenshotOptions, ScreenshotOutputResponse, ScreenshotResult};
use crate::output;
use crate::session::SessionManager;
use crate::window;

/// 发起截图会话（业务层命令，支持 Promise 异步等待直至用户编辑确认或取消）
#[command]
pub async fn capture<R: Runtime>(
    app: AppHandle<R>,
    options: ScreenshotOptions,
) -> Result<ScreenshotResult, Error> {
    crate::logger::write_log("Snapora:Rust", &format!("capture() called with options: {options:?}"));
    // 1. 生成唯一任务 ID (jobId)
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let job_id = format!("job-tauri-{timestamp}");

    // 2. 调用底层原生截屏模块捕获目标屏幕
    let (frame, png_bytes) = capture::capture_screen(&options)?;
    crate::logger::write_log("Snapora:Rust", &format!("capture_screen() success: {}x{}", frame.pixel_size.width, frame.pixel_size.height));

    // 3. 收集屏幕内打开的窗口边界，用于智能窗口吸附
    let window_snaps = capture::collect_window_snaps(&frame.display.bounds);
    crate::logger::write_log("Snapora:Rust", &format!("collect_window_snaps() found {} windows", window_snaps.len()));

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

    // 4. 创建异步结果通道并注册到全局会话管理器，同时暂存二进制图片字节
    let (tx, rx) = oneshot::channel::<ScreenshotResult>();
    let session_mgr = app.state::<SessionManager>();
    session_mgr.start_session(job_id.clone(), payload.clone(), tx);
    session_mgr.store_frame_bytes(png_bytes);

    // 5. 打开全屏透明置顶遮罩窗口
    crate::logger::write_log("Snapora:Rust", &format!("Showing overlay window for job {job_id}..."));
    let _ = window::show_overlay_window(&app, &frame, &payload)?;

    // 6. 异步阻塞等待用户在 Overlay 完成编辑或按 Esc 取消
    crate::logger::write_log("Snapora:Rust", "Waiting for overlay response (rx.await)...");
    let final_result = match rx.await {
        Ok(res) => {
            let status_str = match &res {
                ScreenshotResult::Completed { data, bounds, output, .. } => {
                    format!("completed (bytes: {}, bounds: {bounds:?}, output: {output:?})", data.len())
                }
                ScreenshotResult::Cancelled => "cancelled".to_string(),
                ScreenshotResult::Failed { code, message } => format!("failed: {code} - {message}"),
            };
            crate::logger::write_log("Snapora:Rust", &format!("rx.await 收到结果: {status_str}"));
            res
        }
        Err(err) => {
            crate::logger::write_log("Snapora:Rust", &format!("❌ rx.await 通道被对端 DROP 断开 ({err:?})，降级为 Cancelled"));
            ScreenshotResult::Cancelled
        }
    };
    crate::logger::write_log("Snapora:Rust", "rx.await 流程结束");

    // 7. 关闭遮罩窗口并返回最终结果
    let _ = window::hide_overlay_window(&app);
    crate::logger::write_log("Snapora:Rust", "Overlay window hidden, capture() complete.");
    Ok(final_result)
}

/// 取消当前进行中的截图会话
#[command]
pub async fn cancel_active<R: Runtime>(app: AppHandle<R>) -> Result<bool, Error> {
    crate::logger::write_log("Snapora:Rust", "cancel_active() called");
    let session_mgr = app.state::<SessionManager>();
    session_mgr.cancel_session();
    let _ = window::hide_overlay_window(&app);
    Ok(true)
}

/// Overlay 前端汇报已就绪，补发初始化帧数据以防丢失
#[command]
pub async fn overlay_ready<R: Runtime>(app: AppHandle<R>) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Rust", "overlay_ready() called from frontend Webview");
    let session_mgr = app.state::<SessionManager>();
    if let Some(payload) = session_mgr.get_init_payload() {
        crate::logger::write_log("Snapora:Rust", &format!("Re-emitting plugin:snapora:initialize with job_id: {}", payload.job_id));
        let _ = app.emit("plugin:snapora:initialize", payload);
    } else {
        crate::logger::write_log("Snapora:Rust", "Warning: overlay_ready called but no active session found!");
    }
    Ok(())
}

/// Overlay 前端汇报图像已渲染完成
#[command]
pub async fn overlay_prepared<R: Runtime>(
    _app: AppHandle<R>,
    job_id: Option<String>,
) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Rust", &format!("overlay_prepared() called from frontend: {job_id:?}"));
    Ok(())
}

/// 反馈提示图层已就绪
#[command]
pub async fn feedback_ready<R: Runtime>(_app: AppHandle<R>) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Rust", "feedback_ready() called");
    Ok(())
}

/// 用户在 Overlay 界面点击取消或按下 Esc
#[command]
pub async fn cancel<R: Runtime>(
    app: AppHandle<R>,
    job_id: Option<String>,
) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Rust", &format!("cancel() called from frontend! job_id: {job_id:?}"));
    let session_mgr = app.state::<SessionManager>();
    session_mgr.cancel_session();
    let _ = window::hide_overlay_window(&app);
    Ok(())
}

/// Overlay 上报错误
#[command]
pub async fn report_error<R: Runtime>(
    app: AppHandle<R>,
    payload: serde_json::Value,
) -> Result<(), Error> {
    crate::logger::write_log("Snapora:Rust", &format!("report_error() called from frontend! payload: {payload:?}"));
    let code = payload.get("code").and_then(|v| v.as_str()).unwrap_or("UNKNOWN");
    let message = payload.get("message").and_then(|v| v.as_str()).unwrap_or("Error");

    let session_mgr = app.state::<SessionManager>();
    session_mgr.complete_session(ScreenshotResult::Failed {
        code: code.to_string(),
        message: message.to_string(),
    });
    let _ = window::hide_overlay_window(&app);
    Ok(())
}

/// 从前端载荷中极速提取图片数据（优先 Base64 极速解码，兜底兼容 Array）
fn extract_image_bytes(res_obj: Option<&serde_json::Value>) -> Vec<u8> {
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

/// 执行保存、复制等原生动作并返回响应
#[command]
pub async fn output<R: Runtime>(
    app: AppHandle<R>,
    payload: serde_json::Value,
) -> Result<ScreenshotOutputResponse, Error> {
    let action = payload.get("action").and_then(|v| v.as_str()).unwrap_or("copy");
    crate::logger::write_log("Snapora:Rust", &format!("output() called: action = {action}"));

    // 提取图像数据（优先 Base64，极速）
    let data_bytes = extract_image_bytes(payload.get("result"));
    if !data_bytes.is_empty() {
        let session_mgr = app.state::<SessionManager>();
        session_mgr.store_output_bytes(data_bytes.clone());
    }

    if action == "copy" {
        if !data_bytes.is_empty() {
            output::copy_png_to_clipboard(&data_bytes)?;
            crate::logger::write_log("Snapora:Rust", "output() -> PNG copied to clipboard!");
        }
        return Ok(ScreenshotOutputResponse::Completed {
            action: "copy".to_string(),
            file_path: None,
        });
    }

    if action == "save" {
        let custom_path = payload.get("filePath").and_then(|v| v.as_str()).map(|s| s.to_string());
        let saved_path = output::save_png_to_disk(&data_bytes, custom_path)?;
        crate::logger::write_log("Snapora:Rust", &format!("output() -> PNG saved to {saved_path}"));
        return Ok(ScreenshotOutputResponse::Completed {
            action: "save".to_string(),
            file_path: Some(saved_path),
        });
    }

    if action == "pin" {
        let session_mgr = app.state::<SessionManager>();
        let init_payload = session_mgr.get_init_payload();
        let locale = init_payload.as_ref().and_then(|p| p.options.locale.as_deref());
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

        crate::pinned::create_pinned_window(&app, &data_bytes, &bounds, locale)?;
        crate::logger::write_log("Snapora:Rust", "output() -> Pinned window created successfully!");
        return Ok(ScreenshotOutputResponse::Completed {
            action: "pin".to_string(),
            file_path: None,
        });
    }

    Ok(ScreenshotOutputResponse::Completed {
        action: action.to_string(),
        file_path: None,
    })
}

/// 确认截图最终结果并唤醒会话等待者
#[command]
pub async fn confirm<R: Runtime>(
    app: AppHandle<R>,
    payload: serde_json::Value,
) -> Result<(), Error> {
    // 关键优化：收到 confirm 立即隐藏全屏遮罩并归还系统焦点，彻底消除鼠标消失与卡顿
    let _ = window::hide_overlay_window(&app);

    crate::logger::write_log("Snapora:Rust", &format!("confirm() called with payload: {:?}", payload.get("result").and_then(|r| r.get("status"))));
    let session_mgr = app.state::<SessionManager>();

    let res = payload.get("result");
    let status = res.and_then(|r| r.get("status")).and_then(|s| s.as_str()).unwrap_or("cancelled");

    if status == "completed" {
        if let Some(r) = res {
            let mut data_bytes = extract_image_bytes(Some(r));
            // 若 confirm 阶段未传图片或为空，直接复用 output 阶段缓存的字节，避免重复传输几兆大对象
            if data_bytes.is_empty() {
                if let Some(cached) = session_mgr.get_output_bytes() {
                    data_bytes = cached;
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

            // 注意：复制或保存动作在 output 阶段已由用户点击时执行完毕，此处无需二次重复写入剪贴板
            let output_meta = if action_str == "save" {
                OutputMetadata::Save { file_path: "saved".to_string() }
            } else if action_str == "pin" {
                OutputMetadata::Pin
            } else {
                OutputMetadata::Copy
            };

            session_mgr.complete_session(ScreenshotResult::Completed {
                data: data_bytes,
                mime_type: "image/png".to_string(),
                bounds,
                display_id,
                output: output_meta,
            });
        } else {
            session_mgr.cancel_session();
        }
    } else {
        session_mgr.cancel_session();
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

/// 前端拉取全屏截图原图 Base64 DataURL（作为兜底支持）
#[command]
pub async fn get_frame_image<R: Runtime>(
    app: AppHandle<R>,
) -> Result<String, Error> {
    let session_mgr = app.state::<SessionManager>();
    if let Some(payload) = session_mgr.get_init_payload() {
        if let Some(frame) = payload.frames.first() {
            if frame.data_url.starts_with("data:image/") {
                return Ok(frame.data_url.clone());
            }
        }
    }
    if let Some(bytes) = session_mgr.get_frame_bytes() {
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &bytes);
        return Ok(format!("data:image/png;base64,{b64}"));
    }
    Err(Error::CaptureFailed("未找到当前截屏图像数据".to_string()))
}

/// 在后台静默预热透明截图遮罩窗口（提前加载 WebView2 运行时与 UI 资源，避免冷启动延迟）
#[command]
pub async fn prewarm<R: Runtime>(app: AppHandle<R>) -> Result<(), Error> {
    window::prewarm_overlay_window(&app)
}



