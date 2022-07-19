use std::io::{BufReader, Read, Seek};

use super::base::{ParseError, SectionCommon};

use crate::readers::read_unsigned_leb128;
use crate::wasm_components::types::{MemoryType, VarUInt32};

#[derive(Debug)]
pub struct MemorySection {
    pub common: SectionCommon,
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
        let common = SectionCommon::parse(reader)?;
        if common.id != 5 {
            panic!("This Section is not MemorySection")
        }
        // ここまで共通 //
        let payload = MemorySectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
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
