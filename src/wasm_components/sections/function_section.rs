use std::io::Read;

use super::base::{ParseError, SectionCommon, SectionCommonInterface};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::VarUInt32;

#[derive(Debug)]
pub struct FunctionSection {
    common: SectionCommon,
    payload: FunctionSectionPayload,
}

#[derive(Debug)]
pub struct FunctionSectionPayload {
    count: VarUInt32,
    types: Vec<VarUInt32>, // sequence of indices into the type section
}

impl FunctionSection {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = SectionCommon::parse(reader)?;
        if common.id != 3 {
            return Err(ParseError::FormatError(String::from(
                "This Section is not FunctionSection",
            )));
        }
        // ここまで共通 //

        let payload = FunctionSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }

    /// 関数の数を返す
    pub fn get_num_functions(&self) -> u32 {
        self.payload.count
    }

    /// すべての関数に対する関数型リストへのインデックスを返す
    pub fn get_indice_list(&self) -> Vec<u32> {
        self.payload.types.clone()
    }

    // Utilities

    /// idx番目の関数の関数型リストへのインデックスを返す
    pub fn get_indice(&self, idx: usize) -> Option<u32> {
        self.get_indice_list().get(idx).map(|v| *v)
    }
}

impl Sizeof for FunctionSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl SectionCommonInterface for FunctionSection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
    }
}

impl FunctionSectionPayload {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut types: Vec<VarUInt32> = Vec::new();
        for _ in 0..count {
            let mut ty = 0;
            match read_unsigned_leb128(reader, &mut ty) {
                Ok(_rs) => (/* To check read size */),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            };
            types.push(ty as VarUInt32);
        }

        Ok(Self {
            count: count as VarUInt32,
            types: types,
        })
    }
}

impl Sizeof for FunctionSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_types: u32 = self
            .types
            .iter()
            .map(|x| usage_bytes_leb128_u(*x as u64) as u32)
            .sum();

        sizeof_count + sizeof_types
    }
}
