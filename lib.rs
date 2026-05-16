use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn multiply(a: i32, b:i32) -> i32 {
	a * b
}

#[wasm_bindgen]
pub fn add(a: i32, b:i32) -> i32 {
        a + b
}

#[wasm_bindgen]
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}


#[wasm_bindgen]
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero"); /*macro en Rust que aborta el programa inmediatamente de forma segura. */
    }
    a / b
}

