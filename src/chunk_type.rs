#![allow(unused_variables)]

use std::convert::TryFrom;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ChunkType {
    pub ancillary: u8,
    pub private: u8,
    pub reserved: u8,
    pub safe_to_copy: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.ancillary,
            self.private,
            self.reserved,
            self.safe_to_copy,
        ]
    }
    fn is_valid(&self) -> bool {
        check(self.ancillary)
            && check(self.private)
            && check_capital(self.reserved)
            && check(self.safe_to_copy)
    }
    fn is_critical(&self) -> bool {
        if check_capital(self.ancillary) {
            return true;
        }
        false
    }
    fn is_public(&self) -> bool {
        if check_capital(self.private) {
            return true;
        }
        false
    }
    fn is_reserved_bit_valid(&self) -> bool {
        if check_capital(self.reserved) {
            return true;
        }
        false
    }
    fn is_safe_to_copy(&self) -> bool {
        check_small(self.safe_to_copy)
    }
}
impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let chunk = ChunkType {
            ancillary: value[0],
            private: value[1],
            reserved: value[2],
            safe_to_copy: value[3],
        };

        if chunk.is_valid() {
            Ok(chunk)
        } else {
            Err("Can't convert [u8;4] to ChunkType")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let chunk = ChunkType {
            ancillary: u8::try_from(chars.next().unwrap()).unwrap(),
            private: u8::try_from(chars.next().unwrap()).unwrap(),
            reserved: u8::try_from(chars.next().unwrap()).unwrap(),
            safe_to_copy: u8::try_from(chars.next().unwrap()).unwrap(),
        };
        if check(chunk.ancillary)
            && check(chunk.private)
            && check(chunk.reserved)
            && check(chunk.safe_to_copy)
        {
            Ok(chunk)
        } else {
            Err("Unable to convert String to ChunkType")
        }
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            std::str::from_utf8(&[
                self.ancillary,
                self.private,
                self.reserved,
                self.safe_to_copy,
            ])
            .unwrap()
        )
    }
}

fn check(val: u8) -> bool {
    if check_capital(val) || check_small(val) {
        return true;
    }
    return false;
}
fn check_capital(val: u8) -> bool {
    if val >= 65 && val <= 90 {
        return true;
    }
    return false;
}

fn check_small(val: u8) -> bool {
    if val >= 97 && val <= 122 {
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        println!("Chunk Rust valid? : {}", chunk.is_valid());
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        println!("Chunk Ru1t err? : {}", chunk.is_err());
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
