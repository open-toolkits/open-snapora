pub mod windows;

use crate::models::ScreenshotBounds;

/// 跨平台的底层窗口枚举入口
pub fn enumerate_platform_windows(target_bounds: &ScreenshotBounds) -> Vec<ScreenshotBounds> {
    #[cfg(target_os = "windows")]
    {
        windows::collect_windows_win32(target_bounds)
    }

    #[cfg(not(target_os = "windows"))]
    {
        let mut snaps = Vec::new();
        if let Ok(windows) = xcap::Window::all() {
            for win in windows {
                if win.is_minimized() {
                    continue;
                }

                let title = win.title();
                let app_name = win.app_name();
                let title_trim = title.trim();

                if title_trim.is_empty() {
                    continue;
                }

                let title_lower = title_trim.to_lowercase();
                let app_lower = app_name.to_lowercase();

                if title_lower.contains("snapora")
                    || title_lower.contains("overlay")
                    || app_lower.contains("snapora")
                {
                    continue;
                }

                let w = win.width() as f64;
                let h = win.height() as f64;
                if w < 60.0 || h < 60.0 {
                    continue;
                }
                let x = win.x() as f64;
                let y = win.y() as f64;

                let inter_x1 = x.max(target_bounds.x);
                let inter_y1 = y.max(target_bounds.y);
                let inter_x2 = (x + w).min(target_bounds.x + target_bounds.width);
                let inter_y2 = (y + h).min(target_bounds.y + target_bounds.height);
                let inter_w = inter_x2 - inter_x1;
                let inter_h = inter_y2 - inter_y1;

                if inter_w < 60.0 || inter_h < 60.0 {
                    continue;
                }

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
}
