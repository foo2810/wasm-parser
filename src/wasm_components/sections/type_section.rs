use std::io::{BufReader, Read, Seek};

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{FuncType, VarUInt32};

#[derive(Debug)]
pub struct TypeSection {
    pub common: SectionCommon,
    pub payload: TypeSectionPayload,
}

#[derive(Debug)]
pub struct TypeSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<FuncType>,
}

impl TypeSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = SectionCommon::parse(reader)?;
        if common.id != 1 {
            // panic!("This Section is not TypeSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not TypeSection",
            )));
        }
        // ここまで共通 //

        let payload = TypeSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}

impl Sizeof for TypeSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl TypeSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut func_types: Vec<FuncType> = Vec::new();
        for _ in 0..count {
            func_types.push(FuncType::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: func_types,
        })
    }
}

impl Sizeof for TypeSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count: u32 = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
    }
}
