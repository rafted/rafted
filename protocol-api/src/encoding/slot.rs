use super::{nbt::NBT, varint::VarInt};

pub struct Slot {
    pub present: bool,
    pub item_id: Option<VarInt>,
    pub item_count: Option<u8>,
    pub nbt: Option<NBT>,
}
