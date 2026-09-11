use serde::Serialize;
use thiserror::Error;

/// open-snapora 插件统一的错误类型
#[derive(Debug, Error)]
pub enum Error {
    #[error("截图会话繁忙: {0}")]
    Busy(String),

    #[error("未找到指定的显示器: {0}")]
    DisplayNotFound(String),

    #[error("屏幕图像采集失败: {0}")]
    CaptureFailed(String),

    #[error("操作系统权限被拒绝: {0}")]
    PermissionDenied(String),

    #[error("窗口管理错误: {0}")]
    WindowError(String),

    #[error("输出动作失败: {0}")]
    OutputError(String),

    #[error("Tauri 核心调用错误: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("序列化/反序列化错误: {0}")]
    Json(#[from] serde_json::Error),
}

/// 序列化给前端调用的错误数据结构
#[derive(Serialize)]
pub struct SerializedError {
    pub code: String,
    pub message: String,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let (code, message) = match self {
            Error::Busy(msg) => ("CAPTURE_BUSY", msg.clone()),
            Error::DisplayNotFound(msg) => ("DISPLAY_NOT_FOUND", msg.clone()),
            Error::CaptureFailed(msg) => ("CAPTURE_FAILED", msg.clone()),
            Error::PermissionDenied(msg) => ("PERMISSION_DENIED", msg.clone()),
            Error::WindowError(msg) => ("OVERLAY_LOAD_FAILED", msg.clone()),
            Error::OutputError(msg) => ("EXPORT_FAILED", msg.clone()),
            Error::Tauri(err) => ("CAPTURE_FAILED", err.to_string()),
            Error::Json(err) => ("INVALID_REQUEST", err.to_string()),
        };

        SerializedError {
            code: code.to_string(),
            message,
        }
        .serialize(serializer)
    }
}
