mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("안녕하세요!!");
}

#[wasm_bindgen]
pub fn sum_factorial(x: u64) -> u64 {
    let mut s: u64 = 0;
    for n in 1..=x {
        s = s + n;
    }
    return s;
}
