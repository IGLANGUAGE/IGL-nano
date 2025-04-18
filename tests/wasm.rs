use wasm_bindgen_test::*;
use igl_nano::{add, sum_slice};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[wasm_bindgen_test]
fn test_sum_slice() {
    let data = [1, 2, 3];
    assert_eq!(unsafe { sum_slice(data.as_ptr(), 3) }, 6);
}
