use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::Error;
use crate::image::ImageBuffer;
use super::super::{OutputContext, OutputHandler};

/// 将 PNG 图像字节保存到指定文件路径或默认临时目录
pub fn save_png_to_disk(png_bytes: &[u8], custom_path: Option<String>) -> Result<String, Error> {
    let target_path = match custom_path {
        Some(p) => PathBuf::from(p),
        None => {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            let mut dir = std::env::temp_dir();
            dir.push("open-snapora");
            let _ = fs::create_dir_all(&dir);
            dir.push(format!("snapora_{timestamp}.png"));
            dir
        }
    };

    if let Some(parent) = target_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    fs::write(&target_path, png_bytes)
        .map_err(|e| Error::CaptureFailed(format!("写入截图文件失败: {e}")))?;

    Ok(target_path.to_string_lossy().to_string())
}

/// 文件保存输出处理器
#[derive(Default)]
pub struct FileOutputHandler;

impl OutputHandler for FileOutputHandler {
    fn can_handle(&self, action: &str) -> bool {
        action == "save"
    }

    fn execute(
        &self,
        image: &ImageBuffer,
        context: &OutputContext,
    ) -> Result<Option<String>, Error> {
        let saved_path = save_png_to_disk(&image.bytes, context.file_path.clone())?;
        crate::logger::write_log("Snapora:Output", &format!("FileOutputHandler -> 截图已保存至: {saved_path}"));
        Ok(Some(saved_path))
    }
}
