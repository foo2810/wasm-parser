use std::io::{Read, Seek};

use super::base::{ParseError, SectionCommon, SectionCommonInterface};

use crate::readers::{read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{MemoryType, VarUInt32};

#[derive(Debug)]
pub struct MemorySection {
    common: SectionCommon,
    payload: MemorySectionPayload,
}

#[derive(Debug)]
pub struct MemorySectionPayload {
    count: VarUInt32,
    entries: Vec<MemoryType>,
}

impl MemorySection {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = SectionCommon::parse(reader)?;
        if common.id != 5 {
            panic!("This Section is not MemorySection")
        }
        // ここまで共通 //
        let payload = MemorySectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }

    /// 線形メモリの数を返す
    ///
    /// Wasm v1のMVPモデルでは、1つで固定
    pub fn get_num_memories(&self) -> u32 {
        self.payload.count
    }

    /// 線形メモリのリストを返す
    ///
    /// Wasm v1のMVPモデルでは、1つだけ返す
    pub fn get_memories(&self) -> Vec<&MemoryType> {
        self.payload.entries.iter().collect()
    }

    /// Utilities

    /// idx番目の線形メモリを返す
    pub fn get_memory(&self, idx: usize) -> Option<&MemoryType> {
        self.payload.entries.get(idx)
    }
}

impl Sizeof for MemorySection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl SectionCommonInterface for MemorySection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
    }
}

impl MemorySectionPayload {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let mut entries: Vec<MemoryType> = Vec::new();
        for _ in 0..count {
            entries.push(MemoryType::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: entries,
        })
    }
}

impl Sizeof for MemorySectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
    }
}
