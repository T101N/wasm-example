extern crate wee_alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(s: &str);
}

#[wasm_bindgen]
pub fn message(text: &str) {
    display_numbers();
    log(&format!("Oi! Message: {text}"));
}

fn display_numbers() {
    let numbers = vec!(1, 3, 4, 5, 23, 23, 4 ,2);
    for number in numbers {
        log(&format!("{number}"));
    }
}