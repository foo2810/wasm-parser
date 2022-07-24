use std::io::Read;

use super::base::{ParseError, SectionCommon, SectionCommonInterface};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::VarUInt32;

#[derive(Debug)]
pub struct StartSection {
    common: SectionCommon,
    payload: StartSectionPayload,
}

#[derive(Debug)]
pub struct StartSectionPayload {
    index: VarUInt32,
}

impl StartSection {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        // Common reading in all sections
        let common = SectionCommon::parse(reader)?;
        if common.id != 8 {
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

    //// start関数のインデックスを返す
    pub fn get_start_func_index(&self) -> u32 {
        self.payload.index
    }
}

impl SectionCommonInterface for StartSection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
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
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut index = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(_rs) => (/* To check read size */),
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
