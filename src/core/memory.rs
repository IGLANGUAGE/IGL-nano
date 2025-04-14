/// Безопасный аллокатор WASM-памяти
pub struct WasmMemory {
    buffer: Vec<u8>,
}

impl WasmMemory {
    pub fn new(size: usize) -> Self {
        Self { buffer: vec![0; size] }
    }
}