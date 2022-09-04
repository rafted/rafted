// u8 perhaps?
const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

pub type RestBuffer = Vec<u8>;
pub type BitSet = Vec<bool>;

pub mod entity_metadata;
pub mod nbt;
pub mod position;
pub mod varint;
pub mod varlong;
pub mod slot;
pub mod particle;
