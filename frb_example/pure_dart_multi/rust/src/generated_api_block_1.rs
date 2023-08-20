#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.81.0.

use crate::api_block_1::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

use crate::block_specific_module::for_api_block_1::StructOnlyForBlock1;
use crate::bridge_generated_shares;
use crate::bridge_generated_shares::*;

// Section: wire functions

fn wire_test_inbuilt_type_in_block_1_impl(
    port_: MessagePort,
    a: impl bridge_generated_shares::Wire2Api<i32> + UnwindSafe,
    b: impl bridge_generated_shares::Wire2Api<f32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, f32>(
        WrapInfo {
            debug_name: "test_inbuilt_type_in_block_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_a = a.wire2api();
            let api_b = b.wire2api();
            move |task_callback| Ok(test_inbuilt_type_in_block_1(api_a, api_b))
        },
    )
}
fn wire_test_string_in_block_1_impl(
    port_: MessagePort,
    s: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
    i: impl bridge_generated_shares::Wire2Api<u64> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_string_in_block_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| Ok(test_string_in_block_1(api_s, api_i))
        },
    )
}
fn wire_test_string_in_sync_in_block_1_impl(
    s: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
    i: impl bridge_generated_shares::Wire2Api<u64> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "test_string_in_sync_in_block_1",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            Ok(test_string_in_sync_in_block_1(api_s, api_i))
        },
    )
}
fn wire_test_optional_string_in_block_1_impl(
    port_: MessagePort,
    s: impl bridge_generated_shares::Wire2Api<Option<String>> + UnwindSafe,
    i: impl bridge_generated_shares::Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, Option<String>>(
        WrapInfo {
            debug_name: "test_optional_string_in_block_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| Ok(test_optional_string_in_block_1(api_s, api_i))
        },
    )
}
fn wire_test_optional_string_in_sync_in_block_1_impl(
    s: impl bridge_generated_shares::Wire2Api<Option<String>> + UnwindSafe,
    i: impl bridge_generated_shares::Wire2Api<i32> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "test_optional_string_in_sync_in_block_1",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            Ok(test_optional_string_in_sync_in_block_1(api_s, api_i))
        },
    )
}
fn wire_test_shared_struct_only_for_sync_with_sync_return_in_block_1_impl(
    name: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
    score: impl bridge_generated_shares::Wire2Api<f64> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "test_shared_struct_only_for_sync_with_sync_return_in_block_1",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_name = name.wire2api();
            let api_score = score.wire2api();
            Ok(test_shared_struct_only_for_sync_with_sync_return_in_block_1(api_name, api_score))
        },
    )
}
fn wire_test_all_shared_struct_in_block_1_impl(
    port_: MessagePort,
    custom: impl bridge_generated_shares::Wire2Api<SharedStructInAllBlocks> + UnwindSafe,
    s: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
    i: impl bridge_generated_shares::Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, SharedStructInAllBlocks>(
        WrapInfo {
            debug_name: "test_all_shared_struct_in_block_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| Ok(test_all_shared_struct_in_block_1(api_custom, api_s, api_i))
        },
    )
}
fn wire_test_shared_struct_in_block_1_for_1_and_2_impl(
    port_: MessagePort,
    custom: impl bridge_generated_shares::Wire2Api<SharedStructInBlock1And2> + UnwindSafe,
    s: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
    i: impl bridge_generated_shares::Wire2Api<i32> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, SharedStructInBlock1And2>(
        WrapInfo {
            debug_name: "test_shared_struct_in_block_1_for_1_and_2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| {
                Ok(test_shared_struct_in_block_1_for_1_and_2(
                    api_custom, api_s, api_i,
                ))
            }
        },
    )
}
fn wire_test_cross_shared_struct_in_block_1_for_1_and_2_impl(
    port_: MessagePort,
    custom: impl bridge_generated_shares::Wire2Api<CrossSharedStructInBlock1And2> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_cross_shared_struct_in_block_1_for_1_and_2",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            move |task_callback| Ok(test_cross_shared_struct_in_block_1_for_1_and_2(api_custom))
        },
    )
}
fn wire_test_unique_struct_1_impl(
    port_: MessagePort,
    custom: impl Wire2Api<StructOnlyForBlock1> + UnwindSafe,
    s: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
    i: impl Wire2Api<i8> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, StructOnlyForBlock1>(
        WrapInfo {
            debug_name: "test_unique_struct_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            let api_s = s.wire2api();
            let api_i = i.wire2api();
            move |task_callback| Ok(test_unique_struct_1(api_custom, api_s, api_i))
        },
    )
}
fn wire_test_struct_defined_in_block_1_impl(
    port_: MessagePort,
    custom: impl Wire2Api<StructDefinedInBlock1> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_struct_defined_in_block_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            move |task_callback| Ok(test_struct_defined_in_block_1(api_custom))
        },
    )
}
fn wire_test_enum_defined_in_block_1_impl(
    port_: MessagePort,
    custom: impl Wire2Api<EnumDefinedInBlock1> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_enum_defined_in_block_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_custom = custom.wire2api();
            move |task_callback| Ok(test_enum_defined_in_block_1(api_custom))
        },
    )
}
fn wire_test_list_in_block_1_impl(
    port_: MessagePort,
    shared_structs: impl bridge_generated_shares::Wire2Api<Vec<SharedStructInAllBlocks>> + UnwindSafe,
    strings: impl bridge_generated_shares::Wire2Api<Vec<String>> + UnwindSafe,
    nums: impl bridge_generated_shares::Wire2Api<Vec<i32>> + UnwindSafe,
    weekdays: impl bridge_generated_shares::Wire2Api<Vec<SharedWeekdaysEnumInAllBlocks>> + UnwindSafe,
    struct_list: impl Wire2Api<Vec<StructDefinedInBlock1>> + UnwindSafe,
    enum_list: impl Wire2Api<Vec<EnumDefinedInBlock1>> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_list_in_block_1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_shared_structs = shared_structs.wire2api();
            let api_strings = strings.wire2api();
            let api_nums = nums.wire2api();
            let api_weekdays = weekdays.wire2api();
            let api_struct_list = struct_list.wire2api();
            let api_enum_list = enum_list.wire2api();
            move |task_callback| {
                Ok(test_list_in_block_1(
                    api_shared_structs,
                    api_strings,
                    api_nums,
                    api_weekdays,
                    api_struct_list,
                    api_enum_list,
                ))
            }
        },
    )
}
fn wire_test_method__method__EnumDefinedInBlock1_impl(
    port_: MessagePort,
    that: impl Wire2Api<EnumDefinedInBlock1> + UnwindSafe,
    message: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_method__method__EnumDefinedInBlock1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_message = message.wire2api();
            move |task_callback| Ok(EnumDefinedInBlock1::test_method(&api_that, api_message))
        },
    )
}
fn wire_test_static_method__static_method__EnumDefinedInBlock1_impl(
    port_: MessagePort,
    message: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_static_method__static_method__EnumDefinedInBlock1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_message = message.wire2api();
            move |task_callback| Ok(EnumDefinedInBlock1::test_static_method(api_message))
        },
    )
}
fn wire_test_method__method__StructDefinedInBlock1_impl(
    port_: MessagePort,
    that: impl Wire2Api<StructDefinedInBlock1> + UnwindSafe,
    message: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_method__method__StructDefinedInBlock1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_message = message.wire2api();
            move |task_callback| Ok(StructDefinedInBlock1::test_method(&api_that, api_message))
        },
    )
}
fn wire_test_static_method__static_method__StructDefinedInBlock1_impl(
    port_: MessagePort,
    message: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_static_method__static_method__StructDefinedInBlock1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_message = message.wire2api();
            move |task_callback| Ok(StructDefinedInBlock1::test_static_method(api_message))
        },
    )
}
fn wire_test_method__method__StructOnlyForBlock1_impl(
    port_: MessagePort,
    that: impl Wire2Api<StructOnlyForBlock1> + UnwindSafe,
    message: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
    num: impl bridge_generated_shares::Wire2Api<u16> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_method__method__StructOnlyForBlock1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_that = that.wire2api();
            let api_message = message.wire2api();
            let api_num = num.wire2api();
            move |task_callback| {
                Ok(StructOnlyForBlock1::test_method(
                    &api_that,
                    api_message,
                    api_num,
                ))
            }
        },
    )
}
fn wire_test_static_method__static_method__StructOnlyForBlock1_impl(
    port_: MessagePort,
    message: impl bridge_generated_shares::Wire2Api<String> + UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap::<_, _, _, String>(
        WrapInfo {
            debug_name: "test_static_method__static_method__StructOnlyForBlock1",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_message = message.wire2api();
            move |task_callback| Ok(StructOnlyForBlock1::test_static_method(api_message))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<i8> for i8 {
    fn wire2api(self) -> i8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for StructOnlyForBlock1 {
    fn into_dart(self) -> support::DartAbi {
        vec![
            self.id.into_dart(),
            self.num.into_dart(),
            self.name.into_dart(),
        ]
        .into_dart()
    }
}
impl support::IntoDartExceptPrimitive for StructOnlyForBlock1 {}
impl rust2dart::IntoIntoDart<StructOnlyForBlock1> for StructOnlyForBlock1 {
    fn into_into_dart(self) -> Self {
        self
    }
}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "generated_api_block_1.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "generated_api_block_1.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
