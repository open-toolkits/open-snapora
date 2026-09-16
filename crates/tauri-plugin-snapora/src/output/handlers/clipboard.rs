use std::borrow::Cow;
use arboard::{Clipboard, ImageData};
use crate::error::Error;
use crate::image::ImageBuffer;
use super::super::{OutputContext, OutputHandler};

/// 将 PNG 图像字节解码并复制到系统剪贴板
pub fn copy_png_to_clipboard(png_bytes: &[u8]) -> Result<(), Error> {
    let img = image::load_from_memory(png_bytes)
        .map_err(|e| Error::CaptureFailed(format!("解码截图数据失败: {e}")))?
        .to_rgba8();

    let (width, height) = img.dimensions();
    let raw_bytes = img.into_raw();

    let mut clipboard = Clipboard::new()
        .map_err(|e| Error::CaptureFailed(format!("无法打开系统剪贴板: {e}")))?;

    let img_data = ImageData {
        width: width as usize,
        height: height as usize,
        bytes: Cow::Owned(raw_bytes),
    };

    clipboard.set_image(img_data)
        .map_err(|e| Error::CaptureFailed(format!("写入系统剪贴板失败: {e}")))?;

    Ok(())
}

/// 剪贴板输出处理器
#[derive(Default)]
pub struct ClipboardOutputHandler;

impl OutputHandler for ClipboardOutputHandler {
    fn can_handle(&self, action: &str) -> bool {
        action == "copy"
    }

    fn execute(
        &self,
        image: &ImageBuffer,
        _context: &OutputContext,
    ) -> Result<Option<String>, Error> {
        if !image.bytes.is_empty() {
            copy_png_to_clipboard(&image.bytes)?;
            crate::logger::write_log("Snapora:Output", "ClipboardOutputHandler -> 截图已写入剪贴板");
        }
        Ok(None)
    }
}
