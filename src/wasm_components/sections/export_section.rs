use std::io::{BufReader, Read, Seek};
use std::str;

use super::base::parse_section_common;
use crate::readers::read_x;
use crate::wasm_components::types::{ExternalKind, VarUInt32};

#[derive(Debug)]
pub struct ExportSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: Option<u32>,
    pub name: Option<String>,
    pub payload: ExportSectionPayload,
}

#[derive(Debug)]
pub struct ExportSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<ExportEntry>,
}
#[derive(Debug)]
pub struct ExportEntry {
    pub field_len: VarUInt32,
    pub field_str: String,
    pub kind: ExternalKind,
    pub index: VarUInt32,
}

impl ExportSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // Common reading in all sections
        let common = parse_section_common(reader);
        if common.id != 7 {
            panic!("This Section is not ExportSection");
        }
        // ここまで共通 //

        let payload = ExportSectionPayload::parse(reader);

        Self {
            id: common.id,
            payload_len: common.payload_len,
            name_len: common.name_len,
            name: common.name,
            payload: payload,
        }
    }
}

impl ExportSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut export_entries: Vec<ExportEntry> = Vec::new();

        for _ in 0..count {
            export_entries.push(ExportEntry::parse(reader));
        }

        Self {
            count: count,
            entries: export_entries,
        }
    }
}

impl ExportEntry {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        let field_len = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let field_str = String::from(str::from_utf8(&read_x(reader, field_len as usize)).unwrap());
        let kind = ExternalKind::parse(reader);
        let index = leb128::read::unsigned(reader).unwrap() as VarUInt32;

        Self {
            field_len: field_len,
            field_str: field_str,
            kind: kind,
            index: index,
        }
    }
}
