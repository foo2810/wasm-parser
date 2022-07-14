use std::io::{BufReader, Read, Seek};

use super::base::parse_section_common;
use crate::wasm_components::types::{GlobalType, InitExpr, VarUInt32};

#[derive(Debug)]
pub struct GlobalSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: GlobalSectionPayload,
}

#[derive(Debug)]
pub struct GlobalSectionPayload {
    pub count: VarUInt32,
    pub globals: Vec<GlobalVariable>,
}

#[derive(Debug)]
pub struct GlobalVariable {
    pub type_: GlobalType,
    pub init: InitExpr,
}

impl GlobalSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections //
        let common = parse_section_common(reader);
        if common.id != 6 {
            panic!("This Section is not GlobalSection")
        }
        // ここまで共通 //

        let payload = GlobalSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl GlobalSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut globals: Vec<GlobalVariable> = Vec::new();
        for _ in 0..count {
            globals.push(GlobalVariable::parse(reader));
        }

        Self {
            count: count,
            globals: globals,
        }
    }
}

impl GlobalVariable {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        let global_type = GlobalType::parse(reader);
        let init_expr = InitExpr::parse(reader);

        Self {
            type_: global_type,
            init: init_expr,
        }
    }
}
