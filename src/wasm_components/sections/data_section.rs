use std::io::{BufReader, Read, Seek};

use super::base::parse_section_common;
use crate::readers::read_8;
use crate::wasm_components::types::{InitExpr, VarUInt32};

#[derive(Debug)]
pub struct DataSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
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
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections //
        let common = parse_section_common(reader);
        if common.id != 11 {
            panic!("This Section is not DataSection");
        }
        // ここまで共通 //

        let payload = DataSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl DataSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut entries: Vec<DataSegment> = Vec::new();
        for _ in 0..count {
            entries.push(DataSegment::parse(reader));
        }

        Self {
            count: count,
            entries: entries,
        }
    }
}

impl DataSegment {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let index = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let offset = InitExpr::parse(reader);
        let size = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..size {
            data.push(read_8(reader)[0]);
        }

        Self {
            index: index,
            offset: offset,
            size: size,
            data: data,
        }
    }
}
