use std::iter::repeat;
use wasm_bindgen::prelude::*;
use web_sys::console;

///// INIT /////
// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
///// /INIT /////

#[wasm_bindgen]
pub struct LargeType {
    long_string: String,
}

#[wasm_bindgen]
impl LargeType {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            long_string: repeat('c').take(100_000).collect(),
        }
    }
}

impl Drop for LargeType {
    fn drop(&mut self) {
        console::log_1(&format!("dealloc @ {:?}", self.long_string.as_ptr()).into());
    }
}
