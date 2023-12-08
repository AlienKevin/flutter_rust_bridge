use super::BaseCodec;
use crate::for_generated::vec_from_leak_ptr;
use crate::generalized_isolate::IntoDart;
use crate::platform_types::DartAbi;
use crate::rust2dart::action::Rust2DartAction;
use std::io::Cursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SseCodec;

impl BaseCodec for SseCodec {
    fn encode<T: IntoDart>(data: T, result_code: Rust2DartAction) -> DartAbi {
        todo!()
    }
}

// TODO maybe move
pub struct SseDeserializer {
    // Only to be used for generated code
    pub cursor: Cursor<Vec<u8>>,
    data_len: i32,
}

impl SseDeserializer {
    pub unsafe fn from_wire(ptr: *const u8, rust_vec_len: i32, data_len: i32) -> Self {
        let vec = vec_from_leak_ptr(ptr, rust_vec_len);
        Self {
            cursor: Cursor::new(vec),
            data_len,
        }
    }

    pub fn end(self) {
        assert_eq!(self.data_len as u64, self.cursor.position());
    }
}

pub struct SseSerializer {}
