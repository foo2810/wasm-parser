use std::io::{BufReader, Read, Seek};
use std::str;

use super::base::{parse_section_common, ParseError};

use crate::readers::{read_unsigned_leb128, read_x};
use crate::wasm_components::types::{ExternalKind, GlobalType, MemoryType, TableType, VarUInt32};

#[derive(Debug)]
pub struct ImportSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: ImportSectionPayload,
}

#[derive(Debug)]
pub struct ImportSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<ImportEntry>,
}

#[derive(Debug)]
pub struct ImportEntry {
    pub module_len: VarUInt32,
    pub module_str: String,
    pub field_len: VarUInt32,
    pub field_str: String,
    pub kind: ExternalKind,
    pub type_: TypeEntry,
}

#[derive(Debug)]
pub enum TypeEntry {
    FuncIndex { type_: VarUInt32 },
    TblType { type_: TableType },
    MemType { type_: MemoryType },
    GblType { type_: GlobalType },
}

impl ImportSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // Common reading in all sections
        let common = parse_section_common(reader)?;
        if common.id != 2 {
            // panic!("This Section is not ImportSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not ImportSection",
            )));
        }
        // ここまで共通 //

        let payload = ImportSectionPayload::parse(reader)?;

        Ok(Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        })
    }
}

impl ImportSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut import_entries: Vec<ImportEntry> = Vec::new();
        for _ in 0..count {
            import_entries.push(ImportEntry::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: import_entries,
        })
    }
}

impl ImportEntry {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let mut module_len: u64 = 0;
        match read_unsigned_leb128(reader, &mut module_len) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let module_str = match read_x(reader, module_len as usize) {
            Ok(data) => match str::from_utf8(&data) {
                Ok(s) => String::from(s),
                Err(err) => return Err(ParseError::FormatError(format!("{:?}", err))),
            },
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut field_len = 0;
        match read_unsigned_leb128(reader, &mut field_len) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let field_str = match read_x(reader, field_len as usize) {
            Ok(data) => match str::from_utf8(&data) {
                Ok(s) => String::from(s),
                Err(err) => return Err(ParseError::FormatError(format!("{:?}", err))),
            },
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let kind = ExternalKind::parse(reader)?;
        let type_ = TypeEntry::parse(reader, &kind)?;

        Ok(Self {
            module_len: module_len as VarUInt32,
            module_str: module_str,
            field_len: field_len as VarUInt32,
            field_str: field_str,
            kind: kind,
            type_: type_,
        })
    }
}

impl TypeEntry {
    pub fn parse<R: Read>(
        reader: &mut BufReader<R>,
        kind: &ExternalKind,
    ) -> Result<Self, ParseError> {
        match kind {
            ExternalKind::Function => {
                let mut type_ = 0;
                match read_unsigned_leb128(reader, &mut type_) {
                    Ok(rs) => (/* To check read size */),
                    Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
                };
                Ok(TypeEntry::FuncIndex {
                    type_: type_ as VarUInt32,
                })
            }
            ExternalKind::Table => {
                let table_type = TableType::parse(reader)?;
                Ok(TypeEntry::TblType { type_: table_type })
            }
            ExternalKind::Memory => {
                let memory_type = MemoryType::parse(reader)?;
                Ok(TypeEntry::MemType { type_: memory_type })
            }
            ExternalKind::Global => {
                let global_type = GlobalType::parse(reader)?;
                Ok(TypeEntry::GblType { type_: global_type })
            }
        }
    }
}
