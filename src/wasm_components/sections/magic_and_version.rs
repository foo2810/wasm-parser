use byteorder::{ByteOrder, LittleEndian};
use std::io::{BufReader, Read};

use super::base::ParseError;
use crate::readers::read_32;

#[derive(Debug, Clone)]
pub struct MagicAndVersion {
    // 0x6d736100
    pub magic: [u8; 4],
    pub version: usize,
}
impl MagicAndVersion {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let magic_buf: [u8; 4] = match read_32(reader) {
            Ok(data) => data,
            Err(err) => return Err(ParseError::ReaderError(format!("{}", err))),
        };
        let version_buf: [u8; 4] = match read_32(reader) {
            Ok(data) => data,
            Err(err) => return Err(ParseError::ReaderError(format!("{}", err))),
        };
        let v = LittleEndian::read_u32(&version_buf);

        Ok(Self {
            magic: magic_buf,
            version: v as usize,
        })
    }
}
