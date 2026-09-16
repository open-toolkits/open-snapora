use xcap::Monitor;
use crate::error::Error;
use crate::image::ImageBuffer;
use crate::models::{CaptureDisplay, CapturedImageFrame, ImageMimeType, PixelSize, ScreenshotBounds, ScreenshotOptions};

/// 屏幕捕获服务抽象接口（ScreenCapture）
pub trait ScreenCapture: Send + Sync {
    /// 枚举系统中所有可用的显示器
    fn displays(&self) -> Result<Vec<CaptureDisplay>, Error>;

    /// 截取目标屏幕并返回符合 open-snapora 标准协议的图片帧和图像缓存
    fn capture(&self, options: &ScreenshotOptions) -> Result<(CapturedImageFrame, ImageBuffer), Error>;
}

/// 默认基于 xcap 的高精度极速屏幕截取提供者
#[derive(Default)]
pub struct DefaultScreenCapture;

impl DefaultScreenCapture {
    pub fn new() -> Self {
        Self
    }
}

impl ScreenCapture for DefaultScreenCapture {
    fn displays(&self) -> Result<Vec<CaptureDisplay>, Error> {
        let monitors = Monitor::all().map_err(|err| {
            Error::CaptureFailed(format!("无法枚举系统显示器: {err}"))
        })?;

        let displays = monitors.into_iter().map(|m| {
            CaptureDisplay {
                id: m.id().to_string(),
                bounds: ScreenshotBounds {
                    x: m.x() as f64,
                    y: m.y() as f64,
                    width: m.width() as f64,
                    height: m.height() as f64,
                },
                scale_factor: m.scale_factor() as f64,
            }
        }).collect();

        Ok(displays)
    }

    fn capture(&self, options: &ScreenshotOptions) -> Result<(CapturedImageFrame, ImageBuffer), Error> {
        let t0 = std::time::Instant::now();
        // 1. 枚举当前操作系统中所有连接的显示器
        let monitors = Monitor::all().map_err(|err| {
            Error::CaptureFailed(format!("无法枚举系统显示器: {err}"))
        })?;

        if monitors.is_empty() {
            return Err(Error::DisplayNotFound("系统中未检测到任何可用显示器".to_string()));
        }

        let t1 = std::time::Instant::now();

        // 2. 根据 options.display 解析目标显示器（优先主屏或首选显示器）
        let target_monitor = if let Some(ref target_id) = options.display {
            if target_id == "primary" {
                monitors.iter().find(|m| m.is_primary()).unwrap_or(&monitors[0])
            } else {
                monitors
                    .iter()
                    .find(|m| m.id().to_string() == *target_id)
                    .unwrap_or(&monitors[0])
            }
        } else {
            // 默认选取主屏幕
            monitors.iter().find(|m| m.is_primary()).unwrap_or(&monitors[0])
        };

        // 3. 读取显示器物理/逻辑尺寸与坐标
        let x = target_monitor.x() as f64;
        let y = target_monitor.y() as f64;
        let width = target_monitor.width() as f64;
        let height = target_monitor.height() as f64;
        let scale_factor = target_monitor.scale_factor() as f64;
        let display_id = target_monitor.id().to_string();

        // 4. 调用原生底层 API 捕获高精度屏幕图像
        let rgba_image = target_monitor.capture_image().map_err(|err| {
            Error::CaptureFailed(format!("原生屏幕截取失败: {err}"))
        })?;
        let t2 = std::time::Instant::now();

        let pixel_width = rgba_image.width();
        let pixel_height = rgba_image.height();

        // 5. 编码为超快高品质 JPEG 格式（质量 92，在 3200x2000 高分屏下仅需 15ms，体积约 800KB~1.2MB）
        let mut image_bytes: Vec<u8> = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut image_bytes, 92);
        encoder
            .encode_image(&rgba_image)
            .map_err(|err| Error::CaptureFailed(format!("JPEG 极速图像编码失败: {err}")))?;
        let t3 = std::time::Instant::now();
        let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &image_bytes);
        let data_url = format!("data:image/jpeg;base64,{b64}");
        let t4 = std::time::Instant::now();

        crate::logger::write_log(
            "Snapora:Capture",
            &format!(
                "截屏性能统计: 枚举显示器 {}ms, 原生底层截屏 {}ms, JPEG极速编码 {}ms, Base64编码 {}ms (总计 {}ms, 数据大小 {} KB)",
                (t1 - t0).as_millis(),
                (t2 - t1).as_millis(),
                (t3 - t2).as_millis(),
                (t4 - t3).as_millis(),
                (t4 - t0).as_millis(),
                image_bytes.len() / 1024
            ),
        );

        let buffer = ImageBuffer::new(
            image_bytes,
            ImageMimeType::Jpeg,
            pixel_width,
            pixel_height,
        );

        let frame = CapturedImageFrame {
            kind: "image".to_string(),
            display: CaptureDisplay {
                id: display_id,
                bounds: ScreenshotBounds {
                    x,
                    y,
                    width,
                    height,
                },
                scale_factor,
            },
            data_url,
            pixel_size: PixelSize {
                width: pixel_width,
                height: pixel_height,
            },
        };

        Ok((frame, buffer))
    }
}
