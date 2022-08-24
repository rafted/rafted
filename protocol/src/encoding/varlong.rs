use bytes::{Buf, BufMut};

use super::{CONTINUE_BIT, SEGMENT_BITS};

pub type VarLong = i64;

pub fn read_varlong(mut buf: &[u8]) -> Result<VarLong, String> {
    let mut value: i64 = 0;
    let mut position: i8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = buf.get_u8();

        value |= ((current_byte & SEGMENT_BITS) << position) as i64;

        if (current_byte & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;

        if position >= 64 {
            // TODO: make const or something
            return Err("VarLong is too big".to_string());
        }
    }

    Ok(value)
}

pub fn write_varlong(mut buf: &mut Vec<u8>, mut value: VarLong) {
    loop {
        if (value & (SEGMENT_BITS as i64)) == 0 {
            buf.put_i64(value);
            break;
        }

        buf.put_i64((value & SEGMENT_BITS as i64) | CONTINUE_BIT as i64);

        // value >>>= 7;
        value >>= 7;
    }
}
