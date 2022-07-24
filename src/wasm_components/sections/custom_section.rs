use std::io::{Read, Seek};
use std::str;

use super::base::{ParseError, SectionCommon, SectionCommonInterface};
use super::name_section::*;

use crate::readers::{read_unsigned_leb128, read_x};
use crate::wasm_components::base::Sizeof;

#[derive(Debug)]
pub struct CustomSection {
    common: SectionCommon,
    real_payload_size: u32,
    payload: CustomSectionPayload,
}

#[derive(Debug)]
pub enum CustomSectionPayload {
    Name { payload: NameSectionPayload },
    General { payload: Vec<u8> },
}

impl CustomSection {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let mut common = SectionCommon::parse(reader)?;
        if common.id != 0 {
            // panic!("This Section is not CustomSection");
            return Err(ParseError::FormatError(String::from(
                "This Section is not CustomSection",
            )));
        }

        let mut nl = 0;
        let sizeof_name_len: i64;
        match read_unsigned_leb128(reader, &mut nl) {
            Ok(rs) => sizeof_name_len = rs as i64,
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        }

        let name_len = nl as u32;

        let name: String;
        match read_x(reader, nl as usize) {
            Ok(data) => match str::from_utf8(&data) {
                Ok(s) => name = String::from(s),
                Err(err) => return Err(ParseError::FormatError(format!("{:?}", err))),
            },
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let sizeof_name = name_len as i64;
        let payload_size = common.payload_len as i64 - sizeof_name - sizeof_name_len;

        let payload: CustomSectionPayload;
        if name.eq("name") {
            println!(" > Debug: custom section name: {}", name);
            payload = CustomSectionPayload::Name {
                payload: NameSectionPayload::parse(reader, payload_size as u32)?,
            };
        } else {
            payload = match read_x(reader, payload_size as usize) {
                Ok(data) => CustomSectionPayload::General { payload: data },
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            };
        }

        common.name = Some(name);
        common.name_len = Some(name_len);

        Ok(Self {
            common: common,
            real_payload_size: payload_size as u32,
            payload: payload,
        })
    }

    pub fn get_real_payload_size(&self) -> u32 {
        self.real_payload_size
    }

    pub fn get_payload(&self) -> &CustomSectionPayload {
        &self.payload
    }
}

impl SectionCommonInterface for CustomSection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
    }
}

impl Sizeof for CustomSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = match &self.payload {
            CustomSectionPayload::Name { payload } => payload.sizeof(),
            CustomSectionPayload::General { payload } => payload.len() as u32,
        };

        sizeof_common + sizeof_payload
    }
}

impl Sizeof for CustomSectionPayload {
    fn sizeof(&self) -> u32 {
        match self {
            CustomSectionPayload::General { payload } => payload.len() as u32,
            CustomSectionPayload::Name { payload } => payload.sizeof(),
        }
    }
}
