use bytes::{Buf, BufMut};

use super::{CONTINUE_BIT, SEGMENT_BITS};

pub type VarLong = i64;

pub fn read_varlong(mut buf: &[u8]) -> Result<VarLong, String> {
    let mut value: i64 = 0;
    let mut position: i8 = 0;
    let mut current_byte: u8;

    loop {
        current_byte = buf.get_u8();

        value |= (current_byte as i64 & SEGMENT_BITS as i64) << position;

        if (current_byte as i32 & CONTINUE_BIT) == 0 {
            break;
        }

        position += 7;

        if position >= 64 {
            // TODO: make const or something
            return Err("varlong is too big".to_string());
        }
    }

    Ok(value)
}

pub fn write_varlong(mut buf: &mut Vec<u8>, mut value: VarLong) {
    loop {
        if (value & !(SEGMENT_BITS as i64)) == 0 {
            buf.put_u8(value as u8);
            break;
        }

        buf.put_u8((value as u8 & SEGMENT_BITS as u8) | CONTINUE_BIT as u8);

        value >>= 7;
    }
}

#[cfg(test)]
mod tests {
    use super::write_varlong;

    #[test]
    fn write_postitive() {
        let mut buf = vec![];
        write_varlong(&mut buf, 9223372036854775807);
        assert_eq!(buf, vec![0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x7f]);
    }

    #[test]
    fn write_negative() {
        let mut buf = vec![];
        write_varlong(&mut buf, -9223372036854775808);
        assert_eq!(buf, vec![0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x01]);
    }
}

