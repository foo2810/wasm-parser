use std::io::{BufReader, Read, Seek};

use super::base::{parse_section_common, ParseError};

use crate::readers::read_unsigned_leb128;
use crate::wasm_components::types::{VarUInt32, VarUInt7};

#[derive(Debug)]
pub struct FunctionSection {
    pub id: VarUInt7,
    pub payload_len: VarUInt32,
    pub name_len: Option<VarUInt32>,
    pub name: Option<String>,
    pub payload: FunctionSectionPayload,
}

#[derive(Debug)]
pub struct FunctionSectionPayload {
    pub count: VarUInt32,
    pub types: Vec<VarUInt32>, // sequence of indices into the type section
}

impl FunctionSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = parse_section_common(reader)?;
        if common.id != 3 {
            // panic!("This Section is not FunctionSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not FunctionSection",
            )));
        }
        // ここまで共通 //

        let payload = FunctionSectionPayload::parse(reader)?;

        Ok(Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        })
    }
}

impl FunctionSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut types: Vec<VarUInt32> = Vec::new();
        for _ in 0..count {
            let mut ty = 0;
            match read_unsigned_leb128(reader, &mut ty) {
                Ok(rs) => (/* To check read size */),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            };
            types.push(ty as VarUInt32);
        }

        Ok(Self {
            count: count as VarUInt32,
            types: types,
        })
    }
}
