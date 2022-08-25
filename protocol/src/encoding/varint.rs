use bytes::{Buf, BufMut};

use super::{CONTINUE_BIT, SEGMENT_BITS};

pub type VarInt = i32;

pub fn read_varint(mut buf: &[u8]) -> Result<VarInt, String> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = buf.get_u8();

        value |= (current_byte as i32 & SEGMENT_BITS) << position;

        if (current_byte as i32 & CONTINUE_BIT) == 0 {
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

pub fn write_varint(mut buf: &mut Vec<u8>, mut value: VarInt) {
    loop {
        if (value & !SEGMENT_BITS) == 0 {
            buf.put_u8(value as u8);
            break;
        }

        buf.put_u8((value as u8 & SEGMENT_BITS as u8) | CONTINUE_BIT as u8);

        value >>= 7;
    }
}
