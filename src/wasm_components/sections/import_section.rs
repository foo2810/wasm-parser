use std::io::{BufReader, Read, Seek};
use std::str;

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_unsigned_leb128, read_x, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{ExternalKind, GlobalType, MemoryType, TableType, VarUInt32};

#[derive(Debug)]
pub struct ImportSection {
    pub common: SectionCommon,
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
        let common = SectionCommon::parse(reader)?;
        if common.id != 2 {
            // panic!("This Section is not ImportSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not ImportSection",
            )));
        }
        // ここまで共通 //

        let payload = ImportSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }
}

impl Sizeof for ImportSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
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

impl Sizeof for ImportSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
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

impl Sizeof for ImportEntry {
    fn sizeof(&self) -> u32 {
        let sizeof_module_len = usage_bytes_leb128_u(self.module_len as u64) as u32;
        let sizeof_module_str = self.module_len as u32;
        let sizeof_field_len = usage_bytes_leb128_u(self.field_len as u64) as u32;
        let sizeof_str = self.field_len as u32;
        let sizeof_kind = self.kind.sizeof();
        let sizeof_type = self.type_.sizeof();

        sizeof_module_len
            + sizeof_module_str
            + sizeof_field_len
            + sizeof_str
            + sizeof_kind
            + sizeof_type
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

impl Sizeof for TypeEntry {
    fn sizeof(&self) -> u32 {
        match self {
            TypeEntry::FuncIndex { type_ } => usage_bytes_leb128_u(*type_ as u64) as u32,
            TypeEntry::TblType { type_ } => type_.sizeof(),
            TypeEntry::MemType { type_ } => type_.sizeof(),
            TypeEntry::GblType { type_ } => type_.sizeof(),
        }
    }
}
