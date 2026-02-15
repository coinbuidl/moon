#![allow(dead_code)]

pub fn info(message: impl AsRef<str>) {
    eprintln!("[oc-token-optim] {}", message.as_ref());
}
