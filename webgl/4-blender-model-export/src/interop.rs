use wasm_bindgen::{JsValue, JsCast};

pub fn get_uint8array(data: &[u8]) -> Result<js_sys::Uint8Array, JsValue> {
    let data_location = data.as_ptr() as u32;
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();
    let array = js_sys::Uint8Array::new(&memory_buffer)
        .subarray(data_location, data_location + data.len() as u32);

    Ok(array)
}

pub fn get_float32array(data: &[f32]) -> Result<js_sys::Float32Array, JsValue> {
    // https://www.reddit.com/r/rust/comments/a2ljdb/wasm_webgl_why_is_this_pointer_divided_by_4
    let data_location = data.as_ptr() as u32 / 4;
    let memory_buffer = wasm_bindgen::memory()
        .dyn_into::<js_sys::WebAssembly::Memory>()?
        .buffer();
    let array = js_sys::Float32Array::new(&memory_buffer)
        .subarray(data_location, data_location + data.len() as u32);

    Ok(array)
}
