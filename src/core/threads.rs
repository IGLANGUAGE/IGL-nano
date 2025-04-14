use core::sync::atomic::AtomicBool; // Убрали неиспользуемый Ordering

pub struct ThreadManager {
    _initialized: AtomicBool, // Добавляем _ для подавления предупреждения
}