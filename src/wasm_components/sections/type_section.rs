use std::io::{BufReader, Read, Seek};

use super::base::parse_section_common;
use crate::wasm_components::types::{FuncType, VarUInt32};

#[derive(Debug)]
pub struct TypeSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: TypeSectionPayload,
}

#[derive(Debug)]
pub struct TypeSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<FuncType>,
}

impl TypeSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections //
        let common = parse_section_common(reader);
        if common.id != 1 {
            panic!("This Section is not TypeSection")
        }
        // ここまで共通 //

        let payload = TypeSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl TypeSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut func_types: Vec<FuncType> = Vec::new();

        for _ in 0..count {
            func_types.push(FuncType::parse(reader));
        }

        Self {
            count: count,
            entries: func_types,
        }
    }
}
