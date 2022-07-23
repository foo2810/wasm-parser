use core::panic;
use std::io::{Read, Seek};

use super::base::{ParseError, SectionCommon, SectionCommonInterface};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{InitExpr, VarUInt32};

#[derive(Debug)]
pub struct ElementSection {
    common: SectionCommon,
    payload: ElementSectionPayload,
}

#[derive(Debug)]
pub struct ElementSectionPayload {
    count: VarUInt32,
    entries: Vec<ElementSegment>,
}

#[derive(Debug)]
pub struct ElementSegment {
    index: VarUInt32,
    offset: InitExpr,
    num_elem: VarUInt32,
    elems: Vec<VarUInt32>,
}

impl ElementSection {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
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

    /// elemの個数を返す
    pub fn get_num_elements(&self) -> u32 {
        self.payload.count
    }

    /// elemセグメントのリストを返す
    ///
    /// elemセグメントは一つのテーブルに対応する
    pub fn get_element_list(&self) -> Vec<&ElementSegment> {
        self.payload.entries.iter().collect()
    }

    // Utilities

    /// idx番目のelemセグメント
    pub fn get_element(&self, idx: usize) -> Option<&ElementSegment> {
        self.payload.entries.get(idx)
    }
}

impl Sizeof for ElementSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl SectionCommonInterface for ElementSection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
    }
}

impl ElementSectionPayload {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
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
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
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

    /// 対応するテーブルインデックスを返す
    ///
    /// Wasm v1のMVPモデルでは、1つで固定
    pub fn get_table_index(&self) -> u32 {
        self.index
    }

    /// (not implemented) オフセットを返す(予定)
    pub fn get_offset(&self) -> i32 {
        panic!("not implemented")
    }

    /// elemの個数を返す
    pub fn get_num_elements(&self) -> u32 {
        self.num_elem
    }

    /// elemのリストを返す
    pub fn get_elements(&self) -> Vec<u32> {
        self.elems.clone()
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
