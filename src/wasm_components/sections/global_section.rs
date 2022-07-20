use std::io::{Read, Seek};

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{GlobalType, InitExpr, VarUInt32};

#[derive(Debug)]
pub struct GlobalSection {
    pub common: SectionCommon,
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
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = SectionCommon::parse(reader)?;
        if common.id != 6 {
            // panic!("This Section is not GlobalSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not GlobalSection",
            )));
        }
        // ここまで共通 //

        let payload = GlobalSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}

impl Sizeof for GlobalSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl GlobalSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
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

impl Sizeof for GlobalSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_globals: u32 = self.globals.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_globals
    }
}

impl GlobalVariable {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let global_type = GlobalType::parse(reader)?;
        let init_expr = InitExpr::parse(reader)?;

        Ok(Self {
            type_: global_type,
            init: init_expr,
        })
    }
}

impl Sizeof for GlobalVariable {
    fn sizeof(&self) -> u32 {
        let sizeof_global_type = self.type_.sizeof();
        let sizeof_init = self.init.sizeof();

        sizeof_global_type + sizeof_init
    }
}
