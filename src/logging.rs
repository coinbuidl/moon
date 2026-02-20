#![allow(dead_code)]

pub fn info(message: impl AsRef<str>) {
    eprintln!("[MOON] {}", message.as_ref());
}
