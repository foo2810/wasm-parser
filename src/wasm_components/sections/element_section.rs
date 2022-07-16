use std::io::{BufReader, Read, Seek};

use super::base::{parse_section_common, ParseError};

use crate::readers::read_unsigned_leb128;
use crate::wasm_components::types::{InitExpr, VarUInt32};

#[derive(Debug)]
pub struct ElementSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: ElementSectionPayload,
}

#[derive(Debug)]
pub struct ElementSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<ElementSegment>,
}

#[derive(Debug)]
pub struct ElementSegment {
    pub index: VarUInt32,
    pub offset: InitExpr,
    pub num_elem: VarUInt32,
    pub elems: Vec<VarUInt32>,
}

impl ElementSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections
        let common = parse_section_common(reader)?;
        if common.id != 9 {
            // panic!("This Section is not ElementSection");
            return Err(ParseError::FormatError(String::from(
                "This Section is not ElementSection",
            )));
        }
        // ここまで共通 //

        let payload = ElementSectionPayload::parse(reader)?;

        Ok(Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        })
    }
}

impl ElementSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut element_entries: Vec<ElementSegment> = Vec::new();
        for _ in 0..count {
            element_entries.push(ElementSegment::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: element_entries,
        })
    }
}

impl ElementSegment {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut index: u64 = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let offset = InitExpr::parse(reader)?;

        let mut num_elem: u64 = 0;
        match read_unsigned_leb128(reader, &mut num_elem) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut elems: Vec<VarUInt32> = Vec::new();
        for _ in 0..num_elem {
            let mut e = 0;
            match read_unsigned_leb128(reader, &mut e) {
                Ok(rs) => (/* To check read size */),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            }
            elems.push(e as VarUInt32);
        }

        Ok(Self {
            index: index as VarUInt32,
            offset: offset,
            num_elem: num_elem as VarUInt32,
            elems: elems,
        })
    }
}
