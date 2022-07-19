use std::io::{BufReader, Read, Seek};

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{InitExpr, VarUInt32};

#[derive(Debug)]
pub struct ElementSection {
    pub common: SectionCommon,
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
        let common = SectionCommon::parse(reader)?;
        if common.id != 9 {
            // panic!("This Section is not ElementSection");
            return Err(ParseError::FormatError(String::from(
                "This Section is not ElementSection",
            )));
        }
        // ここまで共通 //

        let payload = ElementSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}

impl Sizeof for ElementSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl ElementSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
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

impl Sizeof for ElementSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
    }
}

impl ElementSegment {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut index: u64 = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let offset = InitExpr::parse(reader)?;

        let mut num_elem: u64 = 0;
        match read_unsigned_leb128(reader, &mut num_elem) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut elems: Vec<VarUInt32> = Vec::new();
        for _ in 0..num_elem {
            let mut e = 0;
            match read_unsigned_leb128(reader, &mut e) {
                Ok(_rs) => (/* To check read size */),
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

impl Sizeof for ElementSegment {
    fn sizeof(&self) -> u32 {
        let sizeof_index = usage_bytes_leb128_u(self.index as u64) as u32;
        let sizeof_offset = self.offset.sizeof();
        let sizeof_num_elem = usage_bytes_leb128_u(self.num_elem as u64) as u32;
        let sizeof_elems: u32 = self
            .elems
            .iter()
            .map(|x| usage_bytes_leb128_u(*x as u64) as u32)
            .sum();

        sizeof_index + sizeof_offset + sizeof_num_elem + sizeof_elems
    }
}
