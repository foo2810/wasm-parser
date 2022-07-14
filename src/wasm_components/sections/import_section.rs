use std::io::{BufReader, Read, Seek};
use std::str;

use super::base::parse_section_common;
use crate::readers::read_x;
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
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> ImportSection {
        // Common reading in all sections
        let common = parse_section_common(reader);
        if common.id != 2 {
            panic!("This Section is not ImportSection")
        }
        // ここまで共通 //

        let payload = ImportSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl ImportSectionPayload {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut import_entries: Vec<ImportEntry> = Vec::new();

        for _ in 0..count {
            import_entries.push(ImportEntry::parse(reader));
        }

        Self {
            count: count,
            entries: import_entries,
        }
    }
}

impl ImportEntry {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let module_len = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let module_str =
            String::from(str::from_utf8(&read_x(reader, module_len as usize)).unwrap());
        let field_len = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let field_str = String::from(str::from_utf8(&read_x(reader, field_len as usize)).unwrap());
        let kind = ExternalKind::parse(reader);
        let type_ = TypeEntry::parse(reader, &kind);

        Self {
            module_len: module_len,
            module_str: module_str,
            field_len: field_len,
            field_str: field_str,
            kind: kind,
            type_: type_,
        }
    }
}

impl TypeEntry {
    pub fn parse<R: Read>(reader: &mut BufReader<R>, kind: &ExternalKind) -> Self {
        match kind {
            ExternalKind::Function => {
                let type_ = leb128::read::unsigned(reader).unwrap() as VarUInt32;
                TypeEntry::FuncIndex { type_: type_ }
            }
            ExternalKind::Table => {
                let table_type = TableType::parse(reader);
                TypeEntry::TblType { type_: table_type }
            }
            ExternalKind::Memory => {
                let memory_type = MemoryType::parse(reader);
                TypeEntry::MemType { type_: memory_type }
            }
            ExternalKind::Global => {
                let global_type = GlobalType::parse(reader);
                TypeEntry::GblType { type_: global_type }
            }
        }
    }
}
