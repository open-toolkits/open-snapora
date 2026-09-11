const COMMANDS: &[&str] = &[
    "capture",
    "cancel_active",
    "overlay_ready",
    "overlay_prepared",
    "feedback_ready",
    "cancel",
    "report_error",
    "output",
    "confirm",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS).build();
}
