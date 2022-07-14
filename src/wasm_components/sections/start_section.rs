use std::io::{BufReader, Read, Seek};

use super::base::parse_section_common;
use crate::wasm_components::types::VarUInt32;

#[derive(Debug)]
pub struct StartSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: StartSectionPayload,
}

#[derive(Debug)]
pub struct StartSectionPayload {
    pub index: VarUInt32,
}

impl StartSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections
        let common = parse_section_common(reader);
        if common.id != 8 {
            panic!("This Section is not StartSection");
        }
        // ここまで共通 //

        let payload = StartSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl StartSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let index = leb128::read::unsigned(reader).unwrap() as VarUInt32;

        Self { index: index }
    }
}
