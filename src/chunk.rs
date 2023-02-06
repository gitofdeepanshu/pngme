#![allow(unused_variables)]
use crc::{Crc, CRC_32_ISO_HDLC};
use std::{
    convert::TryFrom,
    fmt,
    io::{BufReader, Read},
    str::Bytes,
    string::FromUtf8Error,
    sync::Arc,
};

use crate::{chunk_type::ChunkType, Error};
pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chunk {
    pub len: u32,
    pub chunk_type: ChunkType,
    pub chunk_data: Vec<u8>,
    pub crc: u32,
}
impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let chunk_type_bytes: [u8; 4] = value[4..8].try_into().unwrap();

        let len = buffer_u32(value).map_err(|_| "My Error")?;
        let len_usize = usize::try_from(len).map_err(|_| "My Error")? + 8;
        let chunk_type = ChunkType::try_from(chunk_type_bytes).map_err(|_| "My err")?;
        let chunk_data = value[8..len_usize].to_vec();
        let crc = buffer_u32(&value[value.len() - 4..]).map_err(|_| "My Error")?;

        if calculate_crc(&value[4..value.len() - 4]) == crc {
            Ok(Chunk {
                len,
                chunk_type,
                chunk_data,
                crc,
            })
        } else {
            Err("Eror 404")
        }
    }
}
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let mut data_vec = vec![];
        data_vec.extend(chunk_type.bytes());
        data_vec.extend(data.clone());

        Chunk {
            len: u32::try_from(data.len()).unwrap(),
            chunk_type: chunk_type,
            chunk_data: data,
            crc: calculate_crc(&data_vec),
        }
    }
    pub fn length(&self) -> u32 {
        self.len
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &&self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.chunk_data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String, FromUtf8Error> {
        println!(
            "data_asString for :{:?} \t String:{:?}",
            self,
            String::from_utf8(self.chunk_data.clone())
        );
        String::from_utf8(self.chunk_data.clone())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let vec: Vec<u8> = self
            .len
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect();
        vec
    }
}

pub fn buffer_u32(val: &[u8]) -> Result<u32, ()> {
    let mut reader = BufReader::new(val);
    let mut buffer: [u8; 4] = [0, 0, 0, 0];

    reader.read_exact(&mut buffer).map_err(|_| ())?;
    Ok(u32::from_be_bytes(buffer))
}

pub fn calculate_crc(val: &[u8]) -> u32 {
    let checksum_value = CASTAGNOLI.checksum(val);

    return checksum_value;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
