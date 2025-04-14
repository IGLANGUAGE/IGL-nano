# IGL-nano
Минимальное WASM-ядро для максимальной производительности
Minimal WASM core for maximum performance

## Быстрый старт
```powershell
# Установка
cargo build --target wasm32-unknown-unknown

# Запуск демо
cd public
python -m http.server 8000

**B. Добавляем документацию к коду** (`src/lib.rs`):
```rust
//! Минимальное WASM-ядро
//! 
//! # Пример
//! ```no_run
//! #[no_mangle]
//! pub extern "C" fn add(a: i32, b: i32) -> i32 {
//!     a + b
//! }
//! ```

#[doc = "Складывает два числа"]
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}