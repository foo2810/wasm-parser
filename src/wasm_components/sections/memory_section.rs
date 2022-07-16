use std::io::{BufReader, Read, Seek};

use super::base::{parse_section_common, ParseError};

use crate::readers::read_unsigned_leb128;
use crate::wasm_components::types::{MemoryType, VarUInt32};

#[derive(Debug)]
pub struct MemorySection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: MemorySectionPayload,
}

#[derive(Debug)]
pub struct MemorySectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<MemoryType>,
}

impl MemorySection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = parse_section_common(reader)?;
        if common.id != 5 {
            panic!("This Section is not MemorySection")
        }
        // ここまで共通 //
        let payload = MemorySectionPayload::parse(reader)?;

        Ok(Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        })
    }
}

impl MemorySectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let mut entries: Vec<MemoryType> = Vec::new();
        for _ in 0..count {
            entries.push(MemoryType::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: entries,
        })
    }
}
