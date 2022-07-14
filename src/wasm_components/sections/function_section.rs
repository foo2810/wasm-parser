use std::io::{BufReader, Read, Seek};

use super::base::parse_section_common;
use crate::wasm_components::types::VarUInt32;

#[derive(Debug)]
pub struct FunctionSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: FunctionSectionPayload,
}

#[derive(Debug)]
pub struct FunctionSectionPayload {
    pub count: VarUInt32,
    pub types: Vec<VarUInt32>, // sequence of indices into the type section
}

impl FunctionSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections //
        let common = parse_section_common(reader);
        if common.id != 3 {
            panic!("This Section is not FunctionSection")
        }
        // ここまで共通 //

        let payload = FunctionSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl FunctionSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut types: Vec<VarUInt32> = Vec::new();
        for _ in 0..count {
            types.push(leb128::read::unsigned(reader).unwrap() as VarUInt32);
        }

        Self {
            count: count,
            types: types,
        }
    }
}
