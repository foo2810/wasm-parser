use byteorder::{ByteOrder, LittleEndian};
use std::io::Read;

use super::base::ParseError;
use crate::readers::read_32;
use crate::wasm_components::base::Sizeof;

#[derive(Debug, Clone)]
pub struct MagicAndVersion {
    // 0x6d736100
    magic: [u8; 4],
    version: usize,
}
impl MagicAndVersion {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
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

    /// バイナリの先頭に格納されているマジックナンバー(4 bytes)を返す
    pub fn get_magic(&self) -> &[u8; 4] {
        &self.magic
    }

    /// Wasmのバージョンを返す
    pub fn get_version(&self) -> u32 {
        self.version as u32
    }
}

impl Sizeof for MagicAndVersion {
    fn sizeof(&self) -> u32 {
        let sizeof_magic: u32 = 4;
        let sizeof_version = 4;

        sizeof_magic + sizeof_version
    }
}
