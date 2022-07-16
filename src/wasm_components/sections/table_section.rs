use std::io::{BufReader, Read, Seek};

use super::base::{parse_section_common, ParseError};

use crate::readers::read_unsigned_leb128;
use crate::wasm_components::types::{TableType, VarUInt32};

#[derive(Debug)]
pub struct TableSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: TableSectionPayload,
}

#[derive(Debug)]
pub struct TableSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<TableType>,
}

impl TableSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = parse_section_common(reader)?;
        if common.id != 4 {
            // panic!("This Section is not TableSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not TableSection",
            )));
        }
        // ここまで共通 //

        let payload = TableSectionPayload::parse(reader)?;

        Ok(Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        })
    }
}

impl TableSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut entries: Vec<TableType> = Vec::new();
        for _ in 0..count {
            entries.push(TableType::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: entries,
        })
    }
}
