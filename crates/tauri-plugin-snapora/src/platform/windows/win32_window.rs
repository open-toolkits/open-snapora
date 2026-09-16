#[cfg(target_os = "windows")]
pub mod win32_impl {
    use std::ffi::c_void;
    use crate::models::ScreenshotBounds;
    use super::super::dpi::set_per_monitor_v2_dpi_awareness;

    #[repr(C)]
    #[derive(Default, Debug, Clone, Copy)]
    struct RECT {
        left: i32,
        top: i32,
        right: i32,
        bottom: i32,
    }

    type HWND = *mut c_void;
    type BOOL = i32;
    type LPARAM = isize;

    const DWMWA_EXTENDED_FRAME_BOUNDS: u32 = 9;
    const DWMWA_CLOAKED: u32 = 14;
    const GWL_EXSTYLE: i32 = -20;
    const WS_EX_TOOLWINDOW: u32 = 0x00000080;

    extern "system" {
        fn EnumWindows(lpEnumFunc: unsafe extern "system" fn(HWND, LPARAM) -> BOOL, lParam: LPARAM) -> BOOL;
        fn IsWindowVisible(hWnd: HWND) -> BOOL;
        fn IsIconic(hWnd: HWND) -> BOOL;
        fn GetWindowTextLengthW(hWnd: HWND) -> i32;
        fn GetWindowTextW(hWnd: HWND, lpString: *mut u16, nMaxCount: i32) -> i32;
        fn GetClassNameW(hWnd: HWND, lpClassName: *mut u16, nMaxCount: i32) -> i32;
        fn GetWindowLongW(hWnd: HWND, nIndex: i32) -> i32;
        fn GetWindowThreadProcessId(hWnd: HWND, lpdwProcessId: *mut u32) -> u32;
        fn GetCurrentProcessId() -> u32;
        fn GetWindowRect(hWnd: HWND, lpRect: *mut RECT) -> BOOL;
    }

    #[link(name = "dwmapi")]
    extern "system" {
        fn DwmGetWindowAttribute(hwnd: HWND, dwAttribute: u32, pvAttribute: *mut c_void, cbAttribute: u32) -> i32;
    }

    struct EnumState<'a> {
        target_bounds: &'a ScreenshotBounds,
        current_pid: u32,
        snaps: Vec<ScreenshotBounds>,
    }

    unsafe extern "system" fn enum_windows_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let state = &mut *(lparam as *mut EnumState);

        // 1. 过滤隐藏和已最小化的后台窗口
        if IsWindowVisible(hwnd) == 0 || IsIconic(hwnd) != 0 {
            return 1;
        }

        // 2. 过滤 Cloaked 窗口（被虚拟桌面隔离、未激活的 UWP 挂起后台等）
        let mut cloaked = 0u32;
        let dwm_res = DwmGetWindowAttribute(
            hwnd,
            DWMWA_CLOAKED,
            &mut cloaked as *mut _ as *mut c_void,
            std::mem::size_of::<u32>() as u32,
        );
        if dwm_res == 0 && cloaked != 0 {
            return 1;
        }

        // 3. 过滤无标题的浮动工具窗口 (WS_EX_TOOLWINDOW)
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
        let text_len = GetWindowTextLengthW(hwnd);
        if (ex_style & WS_EX_TOOLWINDOW) != 0 && text_len == 0 {
            return 1;
        }

        // 4. 过滤 Windows 系统级非交互窗口类名（Progman 桌面壁纸、WorkerW、任务栏小部件等）
        let mut class_buf = [0u16; 256];
        let class_len = GetClassNameW(hwnd, class_buf.as_mut_ptr(), 256);
        let class_name = if class_len > 0 {
            String::from_utf16_lossy(&class_buf[..class_len as usize])
        } else {
            String::new()
        };

        if class_name == "Progman" || class_name == "WorkerW" {
            return 1;
        }

        // 5. 获取窗口标题并过滤 Overlay 自身遮罩
        let title = if text_len > 0 {
            let mut title_buf = vec![0u16; (text_len + 1) as usize];
            GetWindowTextW(hwnd, title_buf.as_mut_ptr(), text_len + 1);
            String::from_utf16_lossy(&title_buf[..text_len as usize])
        } else {
            String::new()
        };

        let title_trim = title.trim();
        // 过滤系统阴影和空标题内部组件
        if title_trim.is_empty() && (class_name.starts_with("Windows.UI") || class_name.contains("DropShadow") || class_name == "PopupHost") {
            return 1;
        }

        let title_lower = title_trim.to_lowercase();
        // 仅精准过滤 Snapora Overlay 全屏置顶遮罩层自身，不能误伤 Demo 应用或宿主主窗口
        if title == "open-snapora Overlay" || title_lower == "open-snapora overlay" {
            return 1;
        }

        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, &mut pid);

        // 6. 使用 DWMWA_EXTENDED_FRAME_BOUNDS 获取 100% 真实的物理可视边框（自动去除 Windows 10/11 的不可见阴影边距）
        let mut frame_rect = RECT::default();
        let frame_res = DwmGetWindowAttribute(
            hwnd,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut frame_rect as *mut _ as *mut c_void,
            std::mem::size_of::<RECT>() as u32,
        );

        let (left, top, right, bottom) = if frame_res == 0 && frame_rect.right > frame_rect.left && frame_rect.bottom > frame_rect.top {
            (frame_rect.left, frame_rect.top, frame_rect.right, frame_rect.bottom)
        } else {
            let mut win_rect = RECT::default();
            GetWindowRect(hwnd, &mut win_rect);
            (win_rect.left, win_rect.top, win_rect.right, win_rect.bottom)
        };

        let w = (right - left) as f64;
        let h = (bottom - top) as f64;
        let x = left as f64;
        let y = top as f64;

        if w < 50.0 || h < 50.0 {
            return 1;
        }

        // 过滤自身的全屏遮罩窗口（如果尺寸等于整个屏幕且属于当前进程，判定为 Overlay 遮罩层）
        if pid == state.current_pid
            && w >= (state.target_bounds.width - 10.0)
            && h >= (state.target_bounds.height - 10.0)
        {
            return 1;
        }

        // 7. 严格校验与当前目标显示器的可视相交区域（彻底排除在其他屏幕上的窗口）
        let inter_x1 = x.max(state.target_bounds.x);
        let inter_y1 = y.max(state.target_bounds.y);
        let inter_x2 = (x + w).min(state.target_bounds.x + state.target_bounds.width);
        let inter_y2 = (y + h).min(state.target_bounds.y + state.target_bounds.height);
        let inter_w = inter_x2 - inter_x1;
        let inter_h = inter_y2 - inter_y1;

        if inter_w < 50.0 || inter_h < 50.0 {
            return 1;
        }

        crate::logger::write_log(
            "Snapora:Platform:Windows",
            &format!("Win32精准嗅探到顶层窗口 [{title_trim} ({class_name})] 坐标: ({x}, {y}, {w}x{h})"),
        );

        // EnumWindows 按从上到下 (Z-Order) 顺序返回，保证最顶层窗口排在最前面
        state.snaps.push(ScreenshotBounds {
            x,
            y,
            width: w,
            height: h,
        });

        1
    }

    /// 执行 Windows Win32 顶层窗口枚举，返回按 Z-Order 排列的可吸附边界
    pub fn collect_windows_win32(target_bounds: &ScreenshotBounds) -> Vec<ScreenshotBounds> {
        unsafe {
            set_per_monitor_v2_dpi_awareness();
            let current_pid = GetCurrentProcessId();
            let mut state = EnumState {
                target_bounds,
                current_pid,
                snaps: Vec::new(),
            };
            EnumWindows(enum_windows_callback, &mut state as *mut _ as isize);
            state.snaps
        }
    }
}
