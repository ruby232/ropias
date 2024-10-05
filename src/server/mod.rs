use std::error::Error;
use std::thread;
use std::time::Duration;
use arboard::Clipboard;
use crate::db;

pub fn server() -> Result<(), Box<dyn Error>> {
    let mut clipboard = Clipboard::new()?;
    let mut last_content = clipboard.get_text().unwrap_or_default();

    loop {
        let current_content = clipboard.get_text().unwrap_or_default();
        if current_content != last_content {
            db::save_clipboard_content(&current_content)?;
            last_content = current_content;
        }
        thread::sleep(Duration::from_millis(10));
    }
}
