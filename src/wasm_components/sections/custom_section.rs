use std::io::{BufReader, Read, Seek};
use std::str;

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_unsigned_leb128, read_x};
use crate::wasm_components::types::{VarUInt32, VarUInt7};

#[derive(Debug)]
pub struct CustomSection {
    pub common: SectionCommon,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct NameSection {
    pub common: SectionCommon,
    pub payload: NameSectionPayload,
}

#[derive(Debug)]
pub struct NameSectionPayload {
    pub name: VarUInt7,
    pub name_payload_len: VarUInt32,
    pub name_payload_data: Vec<u8>, // Parse for name_payload_data is not implementd yet
}

impl CustomSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let common = SectionCommon::parse(reader)?;
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
        let payload = match read_x(reader, payload_size as usize) {
            Ok(data) => data,
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}
