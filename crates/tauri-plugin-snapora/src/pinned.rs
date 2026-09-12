use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, PhysicalSize, Runtime, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use crate::error::Error;
use crate::models::ScreenshotBounds;
use crate::output;

/// 右键菜单宽度约 144px + 边距，最小限制为 160px，防止缩得比菜单还小
const PINNED_WINDOW_MIN_WIDTH: f64 = 160.0;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinnedMenuLabels {
    pub actions: String,
    pub copy: String,
    pub copied: String,
    pub save: String,
    pub close: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PinnedInitData {
    pub data_base64: String,
    pub mime_type: String,
    pub locale: String,
    pub menu_labels: PinnedMenuLabels,
}

pub struct PinnedSession {
    pub label: String,
    pub image_bytes: Vec<u8>,
    pub data_base64: String,
    pub locale: String,
}

#[derive(Debug, Clone, Copy)]
pub struct DragState {
    pub mouse_x: f64,
    pub mouse_y: f64,
    pub win_x: i32,
    pub win_y: i32,
}

pub struct PinnedManager {
    sessions: Mutex<HashMap<String, PinnedSession>>,
    drag_states: Mutex<HashMap<String, DragState>>,
    counter: AtomicU64,
}

impl PinnedManager {
    pub fn new() -> Self {
        Self {
            sessions: Mutex::new(HashMap::new()),
            drag_states: Mutex::new(HashMap::new()),
            counter: AtomicU64::new(1),
        }
    }

    pub fn next_id(&self) -> u64 {
        self.counter.fetch_add(1, Ordering::SeqCst)
    }

    pub fn insert_session(&self, label: String, session: PinnedSession) {
        if let Ok(mut map) = self.sessions.lock() {
            map.insert(label, session);
        }
    }

    pub fn get_session(&self, label: &str) -> Option<PinnedSession> {
        let map = self.sessions.lock().ok()?;
        map.get(label).map(|s| PinnedSession {
            label: s.label.clone(),
            image_bytes: s.image_bytes.clone(),
            data_base64: s.data_base64.clone(),
            locale: s.locale.clone(),
        })
    }

    pub fn remove_session(&self, label: &str) {
        if let Ok(mut map) = self.sessions.lock() {
            map.remove(label);
        }
        if let Ok(mut states) = self.drag_states.lock() {
            states.remove(label);
        }
    }
}

/// 在用户所截选区的屏幕原位创建独立无边框置顶贴图窗口
pub fn create_pinned_window<R: Runtime>(
    app: &AppHandle<R>,
    image_bytes: &[u8],
    bounds: &ScreenshotBounds,
    locale: Option<&str>,
) -> Result<WebviewWindow<R>, Error> {
    let pinned_mgr = app.state::<PinnedManager>();
    let id = pinned_mgr.next_id();
    let label = format!("snapora-pinned-{id}");

    // 1. 使用选区的纯物理像素尺寸与物理屏幕坐标，1:1 精准对齐截图原位
    let phys_width = bounds.width.round().max(1.0) as u32;
    let phys_height = bounds.height.round().max(1.0) as u32;
    let phys_x = bounds.x.round() as i32;
    let phys_y = bounds.y.round() as i32;

    crate::logger::write_log(
        "Snapora:Rust",
        &format!("创建贴图置顶窗口 [{label}] 物理尺寸: {phys_width}x{phys_height} 物理坐标: ({phys_x}, {phys_y})"),
    );

    // 2. 将图片转为 Base64 字符串供 Webview 极速加载
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, image_bytes);
    let session = PinnedSession {
        label: label.clone(),
        image_bytes: image_bytes.to_vec(),
        data_base64: b64,
        locale: locale.unwrap_or("zh-CN").to_string(),
    };
    pinned_mgr.insert_session(label.clone(), session);

    // 3. 构建无边框、置顶、带阴影的独立贴图 Webview 窗口
    let window = WebviewWindowBuilder::new(
        app,
        &label,
        WebviewUrl::App("overlay/pinned.html".into()),
    )
    .title("贴图 (Pinned)")
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
    .shadow(true)
    .resizable(true)
    .build()
    .map_err(|err| Error::WindowError(format!("创建置顶贴图窗口失败: {err}")))?;

    // 使用物理像素尺寸和坐标，确保高分屏缩放倍率下与截图 1:1 像素完全重合
    let _ = window.set_position(PhysicalPosition::new(phys_x, phys_y));
    let _ = window.set_size(PhysicalSize::new(phys_width, phys_height));

    Ok(window)
}

/// 贴图页面就绪，请求自身图片载荷及本地化配置
#[tauri::command]
pub async fn pinned_ready<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
) -> Result<PinnedInitData, Error> {
    let label = window.label();
    crate::logger::write_log("Snapora:Rust", &format!("pinned_ready() called from [{label}]"));
    let pinned_mgr = app.state::<PinnedManager>();
    let session = pinned_mgr
        .get_session(label)
        .ok_or_else(|| Error::CaptureFailed(format!("未找到贴图窗口 [{label}] 的对应数据")))?;

    let is_zh = session.locale.starts_with("zh");
    let menu_labels = if is_zh {
        PinnedMenuLabels {
            actions: "操作".to_string(),
            copy: "复制 (Ctrl+C)".to_string(),
            copied: "已复制到剪贴板".to_string(),
            save: "保存为文件 (Ctrl+S)".to_string(),
            close: "关闭 (Esc)".to_string(),
        }
    } else {
        PinnedMenuLabels {
            actions: "Actions".to_string(),
            copy: "Copy".to_string(),
            copied: "Copied to clipboard".to_string(),
            save: "Save".to_string(),
            close: "Close".to_string(),
        }
    };

    Ok(PinnedInitData {
        data_base64: session.data_base64,
        mime_type: "image/png".to_string(),
        locale: session.locale,
        menu_labels,
    })
}

/// 贴图右键菜单或快捷键触发复制
#[tauri::command]
pub async fn pinned_copy<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
) -> Result<(), Error> {
    let label = window.label();
    crate::logger::write_log("Snapora:Rust", &format!("pinned_copy() called from [{label}]"));
    let pinned_mgr = app.state::<PinnedManager>();
    if let Some(session) = pinned_mgr.get_session(label) {
        output::copy_png_to_clipboard(&session.image_bytes)?;
        let _ = window.emit("plugin:snapora:pinned_copied", ());
    }
    Ok(())
}

/// 贴图右键菜单或快捷键触发保存
#[tauri::command]
pub async fn pinned_save<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
) -> Result<String, Error> {
    let label = window.label();
    crate::logger::write_log("Snapora:Rust", &format!("pinned_save() called from [{label}]"));
    let pinned_mgr = app.state::<PinnedManager>();
    if let Some(session) = pinned_mgr.get_session(label) {
        let path = output::save_png_to_disk(&session.image_bytes, None)?;
        return Ok(path);
    }
    Err(Error::CaptureFailed("贴图数据不存在".to_string()))
}

/// 关闭并销毁贴图窗口
#[tauri::command]
pub async fn pinned_close<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
) -> Result<(), Error> {
    let label = window.label();
    crate::logger::write_log("Snapora:Rust", &format!("pinned_close() called from [{label}]"));
    let pinned_mgr = app.state::<PinnedManager>();
    pinned_mgr.remove_session(label);
    let _ = window.destroy();
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinnedPoint {
    pub x: f64,
    pub y: f64,
}

/// 开始窗口拖拽
#[tauri::command]
pub async fn pinned_start_drag<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
    point: Option<PinnedPoint>,
) -> Result<(), Error> {
    let label = window.label();
    if let Some(pt) = point {
        let pinned_mgr = app.state::<PinnedManager>();
        if let Ok(pos) = window.outer_position() {
            if let Ok(mut states) = pinned_mgr.drag_states.lock() {
                states.insert(label.to_string(), DragState {
                    mouse_x: pt.x,
                    mouse_y: pt.y,
                    win_x: pos.x,
                    win_y: pos.y,
                });
            }
        }
    }
    let _ = window.start_dragging();
    Ok(())
}

/// 随鼠标移动动态平移窗口
#[tauri::command]
pub async fn pinned_move_drag<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
    point: Option<PinnedPoint>,
) -> Result<(), Error> {
    if let Some(pt) = point {
        let label = window.label();
        let pinned_mgr = app.state::<PinnedManager>();
        let drag_state = {
            let states = pinned_mgr.drag_states.lock().ok();
            states.and_then(|s| s.get(label).copied())
        };

        if let Some(start) = drag_state {
            let scale = window.scale_factor().unwrap_or(1.0);
            let dx = ((pt.x - start.mouse_x) * scale).round() as i32;
            let dy = ((pt.y - start.mouse_y) * scale).round() as i32;
            let new_x = start.win_x + dx;
            let new_y = start.win_y + dy;
            let _ = window.set_position(PhysicalPosition::new(new_x, new_y));
        }
    }
    Ok(())
}

/// 结束拖拽
#[tauri::command]
pub async fn pinned_end_drag<R: Runtime>(
    app: AppHandle<R>,
    window: WebviewWindow<R>,
) -> Result<(), Error> {
    let label = window.label();
    let pinned_mgr = app.state::<PinnedManager>();
    if let Ok(mut states) = pinned_mgr.drag_states.lock() {
        states.remove(label);
    }
    Ok(())
}

#[derive(Debug, Clone, Deserialize)]
pub struct PinnedResizePayload {
    pub width: f64,
    pub height: f64,
}

/// 动态调整贴图窗口尺寸，确保最小宽度不小于右键菜单（160px）
#[tauri::command]
pub async fn pinned_resize<R: Runtime>(
    _app: AppHandle<R>,
    window: WebviewWindow<R>,
    payload: PinnedResizePayload,
) -> Result<(), Error> {
    let scale = window.scale_factor().unwrap_or(1.0);
    let min_width = (PINNED_WINDOW_MIN_WIDTH * scale).round() as u32;
    let min_height = (60.0 * scale).round() as u32;

    let target_w = ((payload.width * scale).round() as u32).max(min_width);
    let target_h = ((payload.height * scale).round() as u32).max(min_height);

    let _ = window.set_size(PhysicalSize::new(target_w, target_h));
    Ok(())
}
