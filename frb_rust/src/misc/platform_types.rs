#[cfg(not(wasm))]
pub type WireSyncReturn = *mut allo_isolate::ffi::DartCObject;

/// cbindgen:ignore
#[cfg(wasm)]
pub type WireSyncReturn = wasm_bindgen::JsValue;
