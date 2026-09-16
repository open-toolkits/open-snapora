use serde::{Deserialize, Serialize};

/// 支持的图像 MIME 规范类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageMimeType {
    #[serde(rename = "image/png")]
    Png,
    #[serde(rename = "image/jpeg")]
    Jpeg,
    #[serde(rename = "image/webp")]
    Webp,
}

impl ImageMimeType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Png => "image/png",
            Self::Jpeg => "image/jpeg",
            Self::Webp => "image/webp",
        }
    }
}

/// 轻量图像引用对象，解耦大型内存 Vec<u8>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageRef {
    pub id: String,
    pub mime_type: ImageMimeType,
    pub width: u32,
    pub height: u32,
}

/// 截图会话的完整生命周期状态机
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SessionStatus {
    Created,
    Capturing,
    OverlayReady,
    Editing,
    Processing,
    Completed,
    Cancelled,
    Failed,
}

/// 屏幕选区/显示器逻辑边界
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenshotBounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

/// 启动截图选项配置
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScreenshotOptions {
    pub display: Option<String>,
    pub tools: Option<Vec<String>>,
    pub default_tool: Option<String>,
    pub show_copy_feedback: Option<bool>,
    pub locale: Option<String>,
}

/// 屏幕显示器元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CaptureDisplay {
    pub id: String,
    pub bounds: ScreenshotBounds,
    pub scale_factor: f64,
}

/// 捕获帧（图片帧）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapturedImageFrame {
    pub kind: String,
    pub display: CaptureDisplay,
    pub data_url: String,
    pub pixel_size: PixelSize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelSize {
    pub width: u32,
    pub height: u32,
}

/// 前端输出动作返回结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum ScreenshotOutputResponse {
    #[serde(rename = "completed")]
    Completed {
        action: String,
        file_path: Option<String>,
    },
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed {
        code: String,
        message: String,
    },
}

/// 截图最终交付结果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum ScreenshotResult {
    #[serde(rename = "completed")]
    Completed {
        data: Vec<u8>,
        mime_type: String,
        bounds: ScreenshotBounds,
        display_id: String,
        output: OutputMetadata,
    },
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "failed")]
    Failed {
        code: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "camelCase")]
pub enum OutputMetadata {
    #[serde(rename = "copy")]
    Copy,
    #[serde(rename = "save")]
    Save { file_path: String },
    #[serde(rename = "pin")]
    Pin,
}

/// 传递给 Overlay 前端的初始化会话载荷
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitPayload {
    pub protocol_version: u32,
    pub job_id: String,
    pub options: ScreenshotOptions,
    pub frames: Vec<CapturedImageFrame>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_snap_regions: Option<Vec<ScreenshotBounds>>,
}
