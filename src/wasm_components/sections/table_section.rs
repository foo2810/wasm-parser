use std::io::{BufReader, Read, Seek};

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{TableType, VarUInt32};

#[derive(Debug)]
pub struct TableSection {
    pub common: SectionCommon,
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
        let common = SectionCommon::parse(reader)?;
        if common.id != 4 {
            // panic!("This Section is not TableSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not TableSection",
            )));
        }
        // ここまで共通 //

        let payload = TableSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}

impl Sizeof for TableSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl TableSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
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

impl Sizeof for TableSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count: u32 = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
    }
}
