#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        // 注册 open-snapora Tauri 原生插件
        .plugin(tauri_plugin_snapora::init())
        .run(tauri::generate_context!())
        .expect("启动 open-snapora Tauri 应用失败");
}
