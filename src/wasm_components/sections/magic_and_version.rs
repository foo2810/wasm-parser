use byteorder::{ByteOrder, LittleEndian};
use std::io::{BufReader, Read};

use crate::readers::read_32;

#[derive(Debug, Clone)]
pub struct MagicAndVersion {
    // 0x6d736100
    pub magic: [u8; 4],
    pub version: usize,
}
impl MagicAndVersion {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let magic_buf: [u8; 4] = read_32(reader);
        let version_buf: [u8; 4] = read_32(reader);
        let v = LittleEndian::read_u32(&version_buf);

        Self {
            magic: magic_buf,
            version: v as usize,
        }
    }
}
