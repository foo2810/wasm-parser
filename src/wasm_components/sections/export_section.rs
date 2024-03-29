use std::io::Read;
use std::str;

use super::base::{ParseError, SectionCommon, SectionCommonInterface};

use crate::readers::{read_unsigned_leb128, read_x, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{ExternalKind, VarUInt32};

#[derive(Debug)]
pub struct ExportSection {
    common: SectionCommon,
    payload: ExportSectionPayload,
}

#[derive(Debug)]
pub struct ExportSectionPayload {
    count: VarUInt32,
    entries: Vec<ExportEntry>,
}
#[derive(Debug)]
pub struct ExportEntry {
    field_len: VarUInt32,
    field_str: String,
    kind: ExternalKind,
    index: VarUInt32,
}

impl ExportSection {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        // Common reading in all sections
        let common = SectionCommon::parse(reader)?;
        if common.id != 7 {
            return Err(ParseError::FormatError(String::from(
                "This Section is not ExportSection",
            )));
        }
        // ここまで共通 //

        let payload = ExportSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }

    /// エクスポートエントリの数
    pub fn get_num_export_entries(&self) -> u32 {
        self.payload.count
    }

    /// エクスポートエントリのリストを返す
    pub fn get_export_entry_list(&self) -> Vec<&ExportEntry> {
        self.payload.entries.iter().collect()
    }

    // Utilities

    /// idx番目のエクスポートエントリを返す
    pub fn get_export_entry(&self, idx: usize) -> Option<&ExportEntry> {
        self.payload.entries.get(idx)
    }
}

impl Sizeof for ExportSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl SectionCommonInterface for ExportSection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
    }
}

impl ExportSectionPayload {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut export_entries: Vec<ExportEntry> = Vec::new();
        for _ in 0..count {
            export_entries.push(ExportEntry::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: export_entries,
        })
    }
}

impl Sizeof for ExportSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
    }
}

impl ExportEntry {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut field_len = 0;
        match read_unsigned_leb128(reader, &mut field_len) {
            Ok(_rs) => (/* To check read size */),
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

        let mut index = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        Ok(Self {
            field_len: field_len as VarUInt32,
            field_str: field_str,
            kind: kind,
            index: index as VarUInt32,
        })
    }

    /// エクスポートされるデータのラベル名(シンボル)
    pub fn get_entry_name(&self) -> &String {
        &self.field_str
    }

    /// エクスポートされるデータの種類
    ///
    /// Function, Table, Memory, Global
    pub fn get_kind(&self) -> &ExternalKind {
        &self.kind
    }

    /// 実体にアクセスするためのインデックス
    pub fn get_index(&self) -> u32 {
        self.index
    }
}

impl Sizeof for ExportEntry {
    fn sizeof(&self) -> u32 {
        let sizeof_field_len = usage_bytes_leb128_u(self.field_len as u64) as u32;
        let sizeof_field_str = self.field_len as u32;
        let sizeof_kind = self.kind.sizeof();
        let sizeof_index = usage_bytes_leb128_u(self.index as u64) as u32;

        sizeof_field_len + sizeof_field_str + sizeof_kind + sizeof_index
    }
}
