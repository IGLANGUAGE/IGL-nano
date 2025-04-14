use core::sync::atomic::{AtomicBool, Ordering};

/// Менеджер потоков WASI
pub struct ThreadManager {
    initialized: AtomicBool,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self { initialized: AtomicBool::new(false) }
    }
}