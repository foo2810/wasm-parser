use std::io::Read;

use super::{
    CodeSection, CustomSection, DataSection, ElementSection, ExportSection, FunctionSection,
    GlobalSection, ImportSection, MemorySection, StartSection, TableSection, TypeSection,
};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::*;

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

pub trait SectionCommonInterface {
    /// 各セクションではこの関数のみを実装すれば良い
    fn get_base(&self) -> &SectionCommon;

    fn get_id(&self) -> u8 {
        self.get_base().id
    }

    fn get_payload_len(&self) -> u32 {
        self.get_base().payload_len
    }

    fn get_name(&self) -> Option<&String> {
        self.get_base().name.as_ref()
    }
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
    pub fn parse<R: Read>(reader: &mut R) -> Result<SectionCommon, ParseError> {
        let mut id = 0; // VarUInt7
        match read_unsigned_leb128(reader, &mut id) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut payload_len = 0;
        match read_unsigned_leb128(reader, &mut payload_len) {
            Ok(_rs) => (/* To check read size */),
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

impl Sizeof for SectionCommon {
    fn sizeof(&self) -> u32 {
        let sizeof_id: u32 = 1;
        let sizeof_payload_len = usage_bytes_leb128_u(self.payload_len as u64) as u32;
        if self.id != 0 {
            sizeof_id + sizeof_payload_len
        } else {
            let sizeof_name_len = usage_bytes_leb128_u(self.name_len.unwrap() as u64) as u32;
            let sizeof_name = self.name.as_ref().unwrap().len() as u32;
            sizeof_id + sizeof_payload_len + sizeof_name_len + sizeof_name
        }
    }
}
