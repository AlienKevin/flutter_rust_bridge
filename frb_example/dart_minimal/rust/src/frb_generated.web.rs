// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 1.82.4.

// Section: imports

use super::*;
use flutter_rust_bridge::for_generated::wasm_bindgen;
use flutter_rust_bridge::for_generated::wasm_bindgen::prelude::*;
use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: impl_wire2api

impl<T> Wire2Api<Option<T>> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<i32> for flutter_rust_bridge::for_generated::wasm_bindgen::JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}

#[no_mangle]
pub extern "C" fn frb_initialize_rust(
    dart_opaque_drop_port: flutter_rust_bridge::for_generated::MessagePort,
    dart_fn_invoke_port: flutter_rust_bridge::for_generated::MessagePort,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.initialize(dart_opaque_drop_port, dart_fn_invoke_port)
}

#[wasm_bindgen]
pub fn wire_minimal_adder(port_: flutter_rust_bridge::for_generated::MessagePort, a: i32, b: i32) {
    wire_minimal_adder_impl(port_, a, b)
}
