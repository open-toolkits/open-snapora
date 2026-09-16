use crate::models::ScreenshotBounds;
use super::provider::WindowProvider;

/// 智能窗口吸附区域计算器（SnapRegionCalculator）
pub struct SnapRegionCalculator;

impl SnapRegionCalculator {
    /// 根据目标屏幕边界和系统顶层窗口列表，计算有效的候选吸附区域
    pub fn calculate<P: WindowProvider + ?Sized>(
        provider: &P,
        target_bounds: &ScreenshotBounds,
    ) -> Vec<ScreenshotBounds> {
        let windows = provider.windows(target_bounds);
        let mut regions = Vec::with_capacity(windows.len());

        for win in windows {
            // 确保窗口具有最小可吸附尺寸 (50x50)
            if win.width < 50.0 || win.height < 50.0 {
                continue;
            }

            // 严格计算窗口与当前截屏目标显示器的相交区域
            let inter_x1 = win.x.max(target_bounds.x);
            let inter_y1 = win.y.max(target_bounds.y);
            let inter_x2 = (win.x + win.width).min(target_bounds.x + target_bounds.width);
            let inter_y2 = (win.y + win.height).min(target_bounds.y + target_bounds.height);

            let inter_w = inter_x2 - inter_x1;
            let inter_h = inter_y2 - inter_y1;

            if inter_w >= 50.0 && inter_h >= 50.0 {
                regions.push(ScreenshotBounds {
                    x: win.x,
                    y: win.y,
                    width: win.width,
                    height: win.height,
                });
            }
        }

        regions
    }
}
