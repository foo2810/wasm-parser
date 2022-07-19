use std::io::{BufReader, Read, Seek};

use super::{
    CodeSection, CustomSection, DataSection, ElementSection, ExportSection, FunctionSection,
    GlobalSection, ImportSection, MemorySection, StartSection, TableSection, TypeSection,
};

use crate::readers::read_unsigned_leb128;
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

pub enum ParseError {
    ReaderError(String),
    FormatError(String),
    UnexpectedError(String),
}

// Common part of all section without CustomSection
#[derive(Debug)]
pub struct SectionCommon {
    pub id: VarUInt7,
    pub payload_len: VarUInt32,
    pub name_len: Option<VarUInt32>,
    pub name: Option<String>,
}

impl SectionCommon {
    // pub fn SectionCommon<R: Read + Seek>(
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<SectionCommon, ParseError> {
        let mut id = 0; // VarUInt7
        match read_unsigned_leb128(reader, &mut id) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut payload_len = 0;
        match read_unsigned_leb128(reader, &mut payload_len) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        Ok(SectionCommon {
            id: id as VarUInt7,
            payload_len: payload_len as VarUInt32,
            name_len: None,
            name: None,
        })
    }
}
