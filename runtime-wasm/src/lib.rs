//! WebAssembly runtime components

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Initialize the runtime
#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Execute Synton bytecode
#[wasm_bindgen]
pub fn execute(bytecode: &[u8]) -> Result<Vec<u8>, JsValue> {
    // TODO: Implement WASM execution
    Ok(vec![])
}

/// Compile Synton source to bytecode
#[wasm_bindgen]
pub fn compile(source: &str) -> Result<Vec<u8>, JsValue> {
    // TODO: Implement compilation
    Ok(vec![])
}
