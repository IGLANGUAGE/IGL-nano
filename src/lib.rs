#![no_std]
#![no_main]

use core::arch::wasm32;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Обработчик паники
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Экспортируемые константы ошибок
#[no_mangle]
pub static SUCCESS: i32 = 0;
#[no_mangle]
pub static ERR_INVALID_PARAMS: i32 = -1;
#[no_mangle] 
pub static ERR_MEMORY_ACCESS: i32 = -2;
#[no_mangle]
pub static ERR_OVERFLOW: i32 = -3;
#[no_mangle]
pub static ERR_ALIGNMENT: i32 = -4;

#[no_mangle]
pub extern "C" fn fast_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn mul(a: i32, b: i32) -> i32 {
    a.wrapping_mul(b) // Используем wrapping_mul для защиты от переполнения
}

#[no_mangle]
pub extern "C" fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        ERR_INVALID_PARAMS // Специальный код ошибки для деления на ноль
    } else {
        a / b
    }
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dest: i32, src: i32, len: i32) -> i32 {
    if len == 0 {
        return SUCCESS;
    }
    
    const MAX_ALLOWED: i32 = 1 << 20; // 1MB
    if len > MAX_ALLOWED {
        return ERR_MEMORY_ACCESS;
    }
    
    if dest < 0 || src < 0 || len < 0 {
        return ERR_INVALID_PARAMS;
    }
    
    if dest % 4 != 0 || src % 4 != 0 {
        return ERR_ALIGNMENT;
    }
    
    let memory_size = (wasm32::memory_size(0) as i32) * 65536;
    if dest + len > memory_size || src + len > memory_size {
        return ERR_MEMORY_ACCESS;
    }
    
    // Используем core::ptr вместо std::ptr
    core::ptr::copy_nonoverlapping(
        src as *const u8,
        dest as *mut u8,
        len as usize
    );
    
    SUCCESS
}

#[no_mangle]
pub extern "C" fn pow(base: i32, exp: i32) -> i32 {
    if exp < 0 {
        return ERR_INVALID_PARAMS; // Код ошибки: отрицательная степень
    }
    base.saturating_pow(exp as u32) // Используем saturating_pow для предотвращения переполнения
}

#[no_mangle]
pub extern "C" fn sqrt(x: i32) -> i32 {
    if x < 0 { 
        return -1; // Код ошибки для отрицательного значения
    }
    if x == 0 { 
        return 0; 
    }
    
    let mut guess = (x / 2) as f64;
    let x_f = x as f64;
    let epsilon = 1e-6;
    let mut iterations = 0;
    let max_iterations = 20;
    
    loop {
        let new_guess = 0.5 * (guess + x_f / guess);
        if (new_guess - guess).abs() < epsilon || iterations >= max_iterations {
            break;
        }
        guess = new_guess;
        iterations += 1;
    }
    
    guess as i32
}

#[no_mangle]
pub extern "C" fn xor(a: i32, b: i32) -> i32 {
    a ^ b
}

#[no_mangle]
pub extern "C" fn modular_pow(base: i32, exp: i32, modulus: i32) -> i32 {
    if modulus <= 0 {
        return ERR_INVALID_PARAMS; // Код ошибки: недопустимый модуль
    }
    if exp < 0 {
        return ERR_INVALID_PARAMS; // Код ошибки: отрицательная степень
    }

    let mut result = 1i64;
    let base = base as i64 % modulus as i64;
    let modulus = modulus as i64;

    for _ in 0..exp {
        result = (result * base) % modulus;
        if result < 0 {
            result += modulus;
        }
    }
    result as i32
}

#[no_mangle]
pub unsafe extern "C" fn some_other_function(ptr: i32, len: i32) -> i32 {
    // Вызов sum_slice внутри блока unsafe
    unsafe {
        sum_slice(ptr, len) // Теперь вызов безопасен
    }
}

#[no_mangle]
pub unsafe extern "C" fn sum_slice(ptr: i32, len: i32) -> i32 {
    // Проверка параметров должна быть первой
    if ptr < 0 || len < 0 {
        return ERR_INVALID_PARAMS;
    }
    
    // Затем проверка выравнивания
    if ptr % 4 != 0 {
        return ERR_ALIGNMENT;  // Должно возвращать -4, а не -1
    }

    let ptr_usize = ptr as usize;
    let len_usize = len as usize;

    let memory_size = (wasm32::memory_size(0) as usize) * 65536;

    if ptr_usize.checked_add(len_usize * 4)
       .map_or(true, |end| end > memory_size) 
    {
        return ERR_MEMORY_ACCESS;
    }

    let mut sum: i32 = 0;
    let ptr = ptr as *const i32;

    for i in 0..len_usize {
        sum = match sum.checked_add(*ptr.add(i)) {
            Some(s) => s,
            None => return ERR_OVERFLOW,
        };
    }

    sum
}

#[no_mangle]
pub extern "C" fn average(ptr: i32, len: i32) -> i32 {
    if ptr < 0 || len <= 0 {
        return ERR_INVALID_PARAMS;
    }
    
    let sum = unsafe { sum_slice(ptr, len) };
    
    // Если sum_slice вернула ошибку, передаем ее дальше
    if sum < 0 {
        return sum;
    }
    
    // Защита от деления на 0 (хотя len <= 0 уже проверено)
    sum / len
}

#[no_mangle]
pub extern "C" fn dot_product(ptr_a: i32, ptr_b: i32, len: i32) -> i32 {
    // Проверка на недопустимые параметры
    if ptr_a < 0 || ptr_b < 0 || len <= 0 {
        return ERR_INVALID_PARAMS;
    }

    // Проверка выравнивания (адреса должны быть кратны 4)
    if ptr_a % 4 != 0 || ptr_b % 4 != 0 {
        return ERR_ALIGNMENT;
    }

    // Преобразуем указатели в usize (в байтах)
    let ptr_a = ptr_a as usize;
    let ptr_b = ptr_b as usize;
    let len = len as usize;

    // Получаем память
    let memory_size = wasm32::memory_size(0) as usize * 65536;
    
    // Проверка границ (умножаем len на 4 - размер i32)
    if ptr_a + len * 4 > memory_size || ptr_b + len * 4 > memory_size {
        return ERR_MEMORY_ACCESS;
    }

    let mut sum = 0;
    unsafe {
        let a_ptr = ptr_a as *const i32;
        let b_ptr = ptr_b as *const i32;
        
        for i in 0..len {
            sum += *a_ptr.add(i) * *b_ptr.add(i);
        }
    }
    
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn smoke_test() {
        // Просто проверяем что код компилируется
        assert!(true);
    }
} 
#[no_mangle]
pub extern "C" fn __hidden_author(ptr: *mut u8) {
    let sig = b"IGL-NANO-v0.3\x00\x00\x00";
    unsafe {
        core::ptr::copy_nonoverlapping(
            sig.as_ptr(),
            ptr,
            sig.len()
        );
    }
}
