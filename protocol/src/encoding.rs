use bytes::Buf;

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub type VarInt = i32;

pub fn read_varint(mut buf: &[u8]) -> Result<VarInt, String> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = buf.get_u8();

        value |= ((current_byte & SEGMENT_BITS) << position) as i32;

        if (current_byte & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;

        if position >= 32 {
            // TODO: make const or something
            return Err("VarInt is too big".to_string());
        }
    }

    Ok(value)
}
