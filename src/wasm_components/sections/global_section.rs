use std::io::{BufReader, Read, Seek};

use super::base::{parse_section_common, ParseError};

use crate::readers::read_unsigned_leb128;
use crate::wasm_components::types::{GlobalType, InitExpr, VarUInt32, VarUInt7};

#[derive(Debug)]
pub struct GlobalSection {
    pub id: VarUInt7,
    pub payload_len: VarUInt32,
    pub name_len: Option<VarUInt32>,
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
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = parse_section_common(reader)?;
        if common.id != 6 {
            // panic!("This Section is not GlobalSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not GlobalSection",
            )));
        }
        // ここまで共通 //

        let payload = GlobalSectionPayload::parse(reader)?;

        Ok(Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        })
    }
}

impl GlobalSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let mut globals: Vec<GlobalVariable> = Vec::new();
        for _ in 0..count {
            globals.push(GlobalVariable::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            globals: globals,
        })
    }
}

impl GlobalVariable {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let global_type = GlobalType::parse(reader)?;
        let init_expr = InitExpr::parse(reader)?;

        Ok(Self {
            type_: global_type,
            init: init_expr,
        })
    }
}
