use std::io::{BufReader, Read, Seek};

use super::{
    CodeSection, CustomSection, DataSection, ElementSection, ExportSection, FunctionSection,
    GlobalSection, ImportSection, MemorySection, StartSection, TableSection, TypeSection,
};
use crate::wasm_components::types::*;

#[macro_export]
macro_rules! create_section_struct {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            id: u8,
            payload_len: u32,
            name_len: Option<u32>,
            name: Option<String>,
            payload: Vec<u8>,
        }
    };
}

#[derive(Debug)]
pub enum Section {
    TypeSection(TypeSection),
    ImportSection(ImportSection),
    FunctionSection(FunctionSection),
    TableSection(TableSection),
    MemorySection(MemorySection),
    GlobalSection(GlobalSection),
    ExportSection(ExportSection),
    StartSection(StartSection),
    ElementSection(ElementSection),
    CodeSection(CodeSection),
    DataSection(DataSection),
    CustomSections(Vec<CustomSection>),
}

// Common part of all section without CustomSection
pub struct SectionCommon {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
}

pub fn parse_section_common<R: Read + Seek>(reader: &mut BufReader<R>) -> SectionCommon {
    let id = leb128::read::unsigned(reader).unwrap() as VarUInt7;
    let payload_len = leb128::read::unsigned(reader).unwrap() as VarUInt32;
    SectionCommon {
        id: id,
        payload_len: payload_len,
        name_len: None,
        name: None,
    }
}
