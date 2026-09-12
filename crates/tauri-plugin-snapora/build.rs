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
    "pinned_ready",
    "pinned_copy",
    "pinned_save",
    "pinned_close",
    "pinned_start_drag",
    "pinned_move_drag",
    "pinned_end_drag",
    "pinned_resize",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
