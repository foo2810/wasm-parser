use std::io::{BufReader, Read, Seek};

use super::base::{ParseError, SectionCommon};
use crate::readers::read_unsigned_leb128;
use crate::wasm_components::code::FunctionBody;
use crate::wasm_components::types::VarUInt32;

#[derive(Debug)]
pub struct CodeSection {
    pub common: SectionCommon,
    pub payload: CodeSectionPayload,
}

#[derive(Debug)]
pub struct CodeSectionPayload {
    pub count: VarUInt32,
    pub bodies: Vec<FunctionBody>,
}

impl CodeSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections
        let common = SectionCommon::parse(reader)?;
        if common.id != 10 {
            // panic!("This Section is not CodeSection");
            return Err(ParseError::FormatError(String::from(
                "This Section is not CodeSection",
            )));
        }
        // ここまで共通 //

        let payload = CodeSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}

impl CodeSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut func_bodies: Vec<FunctionBody> = Vec::new();
        for _ in 0..count {
            func_bodies.push(FunctionBody::parse(reader)?);
        }
        Ok(Self {
            count: count as VarUInt32,
            bodies: func_bodies,
        })
    }
}
