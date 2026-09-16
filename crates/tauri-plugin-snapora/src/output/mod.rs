pub mod handlers;

use std::sync::Arc;
use crate::error::Error;
use crate::image::ImageBuffer;
use crate::models::{ScreenshotBounds, ScreenshotOutputResponse};

pub use handlers::{copy_png_to_clipboard, save_png_to_disk};
pub use handlers::{ClipboardOutputHandler, FileOutputHandler};

/// 输出动作执行上下文
#[derive(Debug, Clone)]
pub struct OutputContext {
    pub job_id: Option<String>,
    pub action: String,
    pub file_path: Option<String>,
    pub bounds: ScreenshotBounds,
    pub locale: Option<String>,
}

/// 输出动作处理器接口（dyn compatible，支持运行时动态注册）
pub trait OutputHandler: Send + Sync {
    /// 检查该处理器是否支持当前 action
    fn can_handle(&self, action: &str) -> bool;

    /// 执行具体的输出动作，成功返回可选的文件保存路径
    fn execute(
        &self,
        image: &ImageBuffer,
        context: &OutputContext,
    ) -> Result<Option<String>, Error>;
}

/// 集中式输出管理器（OutputManager）
/// 
/// 负责注册和分发所有截图导出行为（如剪贴板、本地文件保存、以及未来扩展 OCR、AI 分析、云端上传）
#[derive(Clone)]
pub struct OutputManager {
    handlers: Arc<Vec<Box<dyn OutputHandler>>>,
}

impl Default for OutputManager {
    fn default() -> Self {
        Self::new()
    }
}

impl OutputManager {
    /// 创建并注册默认的一组输出动作处理器
    pub fn new() -> Self {
        let mut handlers: Vec<Box<dyn OutputHandler>> = Vec::new();
        handlers.push(Box::new(ClipboardOutputHandler));
        handlers.push(Box::new(FileOutputHandler));

        Self {
            handlers: Arc::new(handlers),
        }
    }

    /// 统一分发并执行指定的输出动作
    pub fn execute(
        &self,
        image: &ImageBuffer,
        context: &OutputContext,
    ) -> Result<ScreenshotOutputResponse, Error> {
        for handler in self.handlers.iter() {
            if handler.can_handle(&context.action) {
                let file_path = handler.execute(image, context)?;
                return Ok(ScreenshotOutputResponse::Completed {
                    action: context.action.clone(),
                    file_path,
                });
            }
        }

        crate::logger::write_log(
            "Snapora:Output",
            &format!("未找到专门的处理器，动作 [{}] 按默认完成返回", context.action),
        );
        Ok(ScreenshotOutputResponse::Completed {
            action: context.action.clone(),
            file_path: None,
        })
    }
}
