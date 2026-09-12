const COMMANDS: &[&str] = &[
    "capture",
    "prewarm",
    "cancel_active",
    "overlay_ready",
    "overlay_prepared",
    "feedback_ready",
    "cancel",
    "report_error",
    "output",
    "confirm",
    "log_message",
    "get_log_path",
    "get_frame_image",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
