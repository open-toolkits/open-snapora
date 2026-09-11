use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

const APP_NAME: &str = "open-snapora";

/// 获取系统标准 AppData / 应用程序支持日志目录
pub fn get_log_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        if let Ok(app_data) = std::env::var("APPDATA") {
            return PathBuf::from(app_data).join(APP_NAME).join("logs");
        }
        if let Ok(user_profile) = std::env::var("USERPROFILE") {
            return PathBuf::from(user_profile).join("AppData").join("Roaming").join(APP_NAME).join("logs");
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join("Library").join("Application Support").join(APP_NAME).join("logs");
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join(APP_NAME).join("logs");
        }
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(home).join(".config").join(APP_NAME).join("logs");
        }
    }

    PathBuf::from("logs")
}

/// 获取日志文件完整路径
pub fn get_log_file_path() -> PathBuf {
    get_log_dir().join("app.log")
}

const MAX_LOG_LENGTH: usize = 2000;

/// 双写日志：既输出到终端 stderr，又自动持久化追加到系统的 AppData/Roaming/open-snapora/logs/app.log 文件
pub fn write_log(tag: &str, message: &str) {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    // 单行日志最长长度严格限制为 2000 字符，防止大对象把日志撑爆或卡死 IO
    let safe_message = if message.chars().count() > MAX_LOG_LENGTH {
        let truncated: String = message.chars().take(1950).collect();
        format!("{truncated}... [TRUNCATED {} chars]", message.chars().count())
    } else {
        message.to_string()
    };

    let line = format!("[{now}][{tag}] {safe_message}");
    eprintln!("{line}");

    let log_dir = get_log_dir();
    if std::fs::create_dir_all(&log_dir).is_ok() {
        let log_file_path = get_log_file_path();
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&log_file_path) {
            let _ = writeln!(file, "{line}");
        }
    }
}
