use std::io::{BufReader, Read, Seek};

use super::base::parse_section_common;
use crate::wasm_components::code::FunctionBody;
use crate::wasm_components::types::VarUInt32;

#[derive(Debug)]
pub struct CodeSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: CodeSectionPayload,
}

#[derive(Debug)]
pub struct CodeSectionPayload {
    pub count: VarUInt32,
    pub bodies: Vec<FunctionBody>,
}

impl CodeSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections
        let common = parse_section_common(reader);
        if common.id != 10 {
            panic!("This Section is not CodeSection");
        }
        // ここまで共通 //

        let payload = CodeSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl CodeSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut func_bodies: Vec<FunctionBody> = Vec::new();
        for _ in 0..count {
            func_bodies.push(FunctionBody::parse(reader));
        }
        Self {
            count: count,
            bodies: func_bodies,
        }
    }
}