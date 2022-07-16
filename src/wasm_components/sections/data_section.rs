use std::io::{BufReader, Read, Seek};

use super::base::{parse_section_common, ParseError};

use crate::readers::{read_8, read_unsigned_leb128};
use crate::wasm_components::types::{InitExpr, VarUInt32, VarUInt7};

#[derive(Debug)]
pub struct DataSection {
    pub id: VarUInt7,
    pub payload_len: VarUInt32,
    pub name_len: Option<VarUInt32>,
    pub name: Option<String>,
    pub payload: DataSectionPayload,
}

#[derive(Debug)]
pub struct DataSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<DataSegment>,
}

#[derive(Debug)]
pub struct DataSegment {
    pub index: VarUInt32,
    pub offset: InitExpr,
    pub size: VarUInt32, // size of data (bytes)
    pub data: Vec<u8>,
}

impl DataSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = parse_section_common(reader)?;
        if common.id != 11 {
            // panic!("This Section is not DataSection");
            return Err(ParseError::FormatError(String::from(
                "This Section is not DataSection",
            )));
        }
        // ここまで共通 //

        let payload = DataSectionPayload::parse(reader)?;

        Ok(Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        })
    }
}

impl DataSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let mut entries: Vec<DataSegment> = Vec::new();
        for _ in 0..count {
            entries.push(DataSegment::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: entries,
        })
    }
}

impl DataSegment {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut index: u64 = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let offset = InitExpr::parse(reader)?;
        let mut size: u64 = 0;
        match read_unsigned_leb128(reader, &mut size) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..size {
            match read_8(reader) {
                Ok(d) => data.push(d[0]),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            };
        }

        Ok(Self {
            index: index as VarUInt32,
            offset: offset,
            size: size as VarUInt32,
            data: data,
        })
    }
}
