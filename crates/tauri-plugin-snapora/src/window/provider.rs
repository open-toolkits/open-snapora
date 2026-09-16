use crate::models::ScreenshotBounds;
use crate::platform::enumerate_platform_windows;

/// 顶层窗口嗅探服务接口（WindowProvider）
pub trait WindowProvider: Send + Sync {
    /// 遍历枚举当前系统可见窗口并返回按 Z-Order 排列的边界
    fn windows(&self, target_bounds: &ScreenshotBounds) -> Vec<ScreenshotBounds>;
}

/// 默认的窗口提供者，基于平台抽象调用系统底层 API
#[derive(Default)]
pub struct DefaultWindowProvider;

impl DefaultWindowProvider {
    pub fn new() -> Self {
        Self
    }
}

impl WindowProvider for DefaultWindowProvider {
    fn windows(&self, target_bounds: &ScreenshotBounds) -> Vec<ScreenshotBounds> {
        enumerate_platform_windows(target_bounds)
    }
}
