pub mod clipboard;
pub mod file;

pub use clipboard::{copy_png_to_clipboard, ClipboardOutputHandler};
pub use file::{save_png_to_disk, FileOutputHandler};
