pub mod dpi;
pub mod win32_window;

#[cfg(target_os = "windows")]
pub use win32_window::win32_impl::collect_windows_win32;
