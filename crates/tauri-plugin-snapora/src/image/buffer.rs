use crate::models::{ImageMimeType, ImageRef};

/// 内存中真实的图像二进制及其元数据包装，统一管理图片格式与尺寸
#[derive(Debug, Clone)]
pub struct ImageBuffer {
    /// 图像二进制字节数组（例如编码后的 JPEG、PNG、WebP）
    pub bytes: Vec<u8>,
    /// 规范的图像 MIME 类型
    pub mime_type: ImageMimeType,
    /// 物理像素宽度
    pub width: u32,
    /// 物理像素高度
    pub height: u32,
}

impl ImageBuffer {
    /// 创建新的图像缓冲区对象
    pub fn new(bytes: Vec<u8>, mime_type: ImageMimeType, width: u32, height: u32) -> Self {
        Self {
            bytes,
            mime_type,
            width,
            height,
        }
    }

    /// 生成对应的轻量 ImageRef 引用对象，用于 Session 协议层传递
    pub fn to_ref(&self, id: String) -> ImageRef {
        ImageRef {
            id,
            mime_type: self.mime_type.clone(),
            width: self.width,
            height: self.height,
        }
    }

    /// 字节大小（字节数）
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}
