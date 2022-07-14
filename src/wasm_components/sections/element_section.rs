use std::io::{BufReader, Read, Seek};

use super::base::parse_section_common;
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
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections
        let common = parse_section_common(reader);
        if common.id != 9 {
            panic!("This Section is not ElementSection");
        }
        // ここまで共通 //

        let payload = ElementSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl ElementSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut element_entries: Vec<ElementSegment> = Vec::new();

        for _ in 0..count {
            element_entries.push(ElementSegment::parse(reader));
        }

        Self {
            count: count,
            entries: element_entries,
        }
    }
}

impl ElementSegment {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let index = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let offset = InitExpr::parse(reader);
        let num_elem = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut elems: Vec<VarUInt32> = Vec::new();
        for _ in 0..num_elem {
            elems.push(leb128::read::unsigned(reader).unwrap() as VarUInt32);
        }

        Self {
            index: index,
            offset: offset,
            num_elem: num_elem,
            elems: elems,
        }
    }
}
