use std::io::{Read, Seek};

use super::base::{ParseError, SectionCommon, SectionCommonInterface};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{FuncType, VarUInt32};

#[derive(Debug)]
pub struct TypeSection {
    common: SectionCommon,
    payload: TypeSectionPayload,
}

#[derive(Debug)]
pub struct TypeSectionPayload {
    count: VarUInt32,
    entries: Vec<FuncType>,
}

impl TypeSection {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = SectionCommon::parse(reader)?;
        if common.id != 1 {
            // panic!("This Section is not TypeSection")
            return Err(ParseError::FormatError(String::from(
                "This Section is not TypeSection",
            )));
        }
        // ここまで共通 //

        let payload = TypeSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }

    /// 関数の型の数を返す
    pub fn get_num_types(&self) -> u32 {
        self.payload.count
    }

    /// 関数の型のリストを返す
    pub fn get_type_list(&self) -> Vec<&FuncType> {
        self.payload.entries.iter().collect()
    }

    // Utilities

    /// idx番目の関数型を返す
    pub fn get_type(&self, idx: usize) -> Option<&FuncType> {
        self.get_type_list().get(idx).map(|v| *v)
    }
}

impl Sizeof for TypeSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl SectionCommonInterface for TypeSection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
    }
}

impl TypeSectionPayload {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut func_types: Vec<FuncType> = Vec::new();
        for _ in 0..count {
            func_types.push(FuncType::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: func_types,
        })
    }
}

impl Sizeof for TypeSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count: u32 = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
    }
}
