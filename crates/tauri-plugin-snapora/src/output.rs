use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use arboard::{Clipboard, ImageData};
use crate::error::Error;

/// 将导出的 PNG 图像数据解码并写入操作系统剪贴板
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

/// 将截图保存到本地磁盘文件
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
