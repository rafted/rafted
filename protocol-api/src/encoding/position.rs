#[derive(PartialEq, Eq, Debug)]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Position {
    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn encode(&self) -> u64 {
        todo!()
    }
}

impl From<u64> for Position {
    fn from(_encoded: u64) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::Position;

    #[test]
    fn encode_decode_positive() {
        let original = Position::new(100, 432, 912);
        let encoded = original.encode();

        let decoded: Position = encoded.into();

        assert_eq!(original, decoded);
    }

    #[test]
    fn encode_decode_negative() {
        let original = Position::new(-100, -432, -912);
        let encoded = original.encode();

        let decoded: Position = encoded.into();

        assert_eq!(original, decoded);
    }


}
