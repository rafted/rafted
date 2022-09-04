use std::num::TryFromIntError;

#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

impl Position {
    pub fn new(x: i32, y: i16, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn encode(&self) -> i64 {
        let x = i64::from(self.x);
        let y = i64::from(self.y);
        let z = i64::from(self.z);

        ((x & 0x3FFFFFF) << 38) | ((z & 0x3FFFFFF) << 12) | (y & 0xFFF)
    }
}

impl TryFrom<i64> for Position {
    type Error = TryFromIntError;

    fn try_from(encoded: i64) -> Result<Self, Self::Error> {
        let x: i32 = (encoded >> 38).try_into()?;
        let y: i16 = (encoded << 52 >> 52).try_into()?;
        let z: i32 = (encoded << 26 >> 38).try_into()?;

        Ok(Self::new(x, y, z))
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn encode_decode_positive() {
        let original = Position::new(100, 432, 912);
        let encoded = original.encode();

        let decoded: Result<Position, _> = encoded.try_into();

        assert!(decoded.is_ok());
        assert_eq!(original, decoded.unwrap());
    }

    #[test]
    fn encode_decode_negative() {
        let original = Position::new(-100, -432, -912);
        let encoded = original.encode();

        let decoded: Result<Position, _> = encoded.try_into();

        assert!(decoded.is_ok());
        assert_eq!(original, decoded.unwrap());
    }
}
