use std::io::{BufReader, Read, Seek};

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::VarUInt32;

#[derive(Debug)]
pub struct StartSection {
    pub common: SectionCommon,
    pub payload: StartSectionPayload,
}

#[derive(Debug)]
pub struct StartSectionPayload {
    pub index: VarUInt32,
}

impl StartSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections
        let common = SectionCommon::parse(reader)?;
        if common.id != 8 {
            // panic!("This Section is not StartSection");
            return Err(ParseError::FormatError(String::from(
                "This Section is not StartSection",
            )));
        }
        // ここまで共通 //

        let payload = StartSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}

impl Sizeof for StartSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl StartSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut index = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        Ok(Self {
            index: index as VarUInt32,
        })
    }
}

impl Sizeof for StartSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_index = usage_bytes_leb128_u(self.index as u64) as u32;

        sizeof_index
    }
}
