// u8 perhaps?
const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub mod bitset;
pub mod entity_metadata;
pub mod nbt;
pub mod particle;
pub mod position;
pub mod rest_buffer;
pub mod slot;
pub mod varint;
pub mod varlong;
pub mod string;

pub mod prelude;
