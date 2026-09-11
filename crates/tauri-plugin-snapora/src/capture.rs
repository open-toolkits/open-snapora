use xcap::Monitor;
use crate::error::Error;
use crate::models::{CaptureDisplay, CapturedImageFrame, PixelSize, ScreenshotBounds, ScreenshotOptions};

/// 捕获目标屏幕并返回符合 open-snapora 标准协议的图片帧
pub fn capture_screen(options: &ScreenshotOptions) -> Result<(CapturedImageFrame, Vec<u8>), Error> {
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

    // 5. 编码为极速无损 PNG 格式（使用 Fast 压缩与 NoFilter 避免逐行多重滤镜运算）
    let mut png_bytes: Vec<u8> = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new_with_quality(
        &mut png_bytes,
        image::codecs::png::CompressionType::Fast,
        image::codecs::png::FilterType::NoFilter,
    );
    use image::ImageEncoder;
    encoder
        .write_image(
            rgba_image.as_raw(),
            pixel_width,
            pixel_height,
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|err| Error::CaptureFailed(format!("PNG 极速图像编码失败: {err}")))?;
    let t3 = std::time::Instant::now();

    crate::logger::write_log(
        "Snapora:Rust",
        &format!(
            "截屏性能统计: 枚举显示器 {}ms, 原生底层截屏 {}ms, PNG极速编码 {}ms (总计 {}ms, 数据大小 {} KB)",
            (t1 - t0).as_millis(),
            (t2 - t1).as_millis(),
            (t3 - t2).as_millis(),
            (t3 - t0).as_millis(),
            png_bytes.len() / 1024
        ),
    );

    Ok((
        CapturedImageFrame {
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
            data_url: "tauri-blob".to_string(),
            pixel_size: PixelSize {
                width: pixel_width,
                height: pixel_height,
            },
        },
        png_bytes,
    ))
}

/// 遍历枚举当前系统所有可见窗口并转换为空闲吸附边界候选（严格保留由前至后的 Z-Order 层级）
pub fn collect_window_snaps(target_bounds: &ScreenshotBounds) -> Vec<ScreenshotBounds> {
    let mut snaps = Vec::new();
    if let Ok(windows) = xcap::Window::all() {
        for win in windows {
            if win.is_minimized() {
                continue;
            }

            let title = win.title();
            let app_name = win.app_name();
            let title_trim = title.trim();

            // 过滤无标题无意义的后台/影子/辅助窗口（如输入法候选框、阴影层等）
            if title_trim.is_empty() {
                continue;
            }

            let title_lower = title_trim.to_lowercase();
            let app_lower = app_name.to_lowercase();

            // 过滤 Snapora 自身窗口与遮罩
            if title_lower.contains("snapora")
                || title_lower.contains("overlay")
                || app_lower.contains("snapora")
            {
                continue;
            }

            let w = win.width() as f64;
            let h = win.height() as f64;
            // 过滤无意义微型窗口
            if w < 60.0 || h < 60.0 {
                continue;
            }
            let x = win.x() as f64;
            let y = win.y() as f64;

            // 检查窗口在目标显示器内的可视交集面积（防止把邻屏只有几像素边缘的窗口嗅探进来）
            let inter_x1 = x.max(target_bounds.x);
            let inter_y1 = y.max(target_bounds.y);
            let inter_x2 = (x + w).min(target_bounds.x + target_bounds.width);
            let inter_y2 = (y + h).min(target_bounds.y + target_bounds.height);
            let inter_w = inter_x2 - inter_x1;
            let inter_h = inter_y2 - inter_y1;

            if inter_w < 60.0 || inter_h < 60.0 {
                continue;
            }

            crate::logger::write_log(
                "Snapora:Rust",
                &format!("嗅探到顶层候选窗口 [{title_trim} / {app_name}] 坐标: ({x}, {y}, {w}x{h})"),
            );

            snaps.push(ScreenshotBounds {
                x,
                y,
                width: w,
                height: h,
            });
        }
    }
    snaps
}
