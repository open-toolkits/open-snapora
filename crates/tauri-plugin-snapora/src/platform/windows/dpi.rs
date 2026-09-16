#[cfg(target_os = "windows")]
pub fn set_per_monitor_v2_dpi_awareness() {
    // -4 对应 DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2，确保返回纯物理像素，与截屏底图 1:1 精确对齐
    const DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2: isize = -4;

    extern "system" {
        fn SetThreadDpiAwarenessContext(dpiContext: isize) -> isize;
    }

    unsafe {
        let _ = SetThreadDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
}
