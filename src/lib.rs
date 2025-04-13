#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! { loop {} }

#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sum_slice(offset: i32, len: i32) -> i32 {
    let mut sum = 0;
    unsafe {
        let ptr = offset as *const i32;
        for i in 0..len {
            sum += *ptr.add(i as usize);
        }
    }
    sum
}