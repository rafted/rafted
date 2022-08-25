use bytes::{Buf, BufMut};

use super::{CONTINUE_BIT, SEGMENT_BITS};

pub type VarInt = i32;

pub fn read_varint(mut buf: &[u8]) -> Result<VarInt, String> {
    let mut value: i32 = 0;
    let mut position: i8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = buf.get_u8();

        value |= ((current_byte & SEGMENT_BITS) as i32) << position;

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

pub fn write_varint(mut buf: &mut Vec<u8>, mut value: VarInt) {
    loop {
        if (value & (SEGMENT_BITS as i32)) == 0 {
            buf.put_i32(value);
            break;
        }

        buf.put_i32((value & SEGMENT_BITS as i32) | CONTINUE_BIT as i32);

        // value >>>= 7;
        value >>= 7;
    }
}
