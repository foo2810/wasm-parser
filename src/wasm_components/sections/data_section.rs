use std::io::{Read, Seek};

use super::base::{ParseError, SectionCommon, SectionCommonInterface};

use crate::readers::{read_8, read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{InitExpr, VarUInt32};

#[derive(Debug)]
pub struct DataSection {
    common: SectionCommon,
    payload: DataSectionPayload,
}

#[derive(Debug)]
pub struct DataSectionPayload {
    count: VarUInt32,
    entries: Vec<DataSegment>,
}

#[derive(Debug)]
pub struct DataSegment {
    index: VarUInt32,
    offset: InitExpr,
    size: VarUInt32, // size of data (bytes)
    data: Vec<u8>,
}

impl DataSection {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        // Common reading in all sections //
        let common = SectionCommon::parse(reader)?;
        if common.id != 11 {
            // panic!("This Section is not DataSection");
            return Err(ParseError::FormatError(String::from(
                "This Section is not DataSection",
            )));
        }
        // ここまで共通 //

        let payload = DataSectionPayload::parse(reader)?;

        Ok(Self {
            common: common,
            payload: payload,
        })
    }

    /// dataの個数を返す
    pub fn get_num_data_segments(&self) -> u32 {
        self.payload.count
    }

    /// dataセグメントのリストを返す
    pub fn get_data_segment_list(&self) -> Vec<&DataSegment> {
        self.payload.entries.iter().collect()
    }

    // Utilities

    /// idx番目のdataセグメント
    pub fn get_data_segment(&self, idx: usize) -> Option<&DataSegment> {
        self.payload.entries.get(idx)
    }
}

impl Sizeof for DataSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
    }
}

impl SectionCommonInterface for DataSection {
    fn get_base(&self) -> &SectionCommon {
        &self.common
    }
}

impl DataSectionPayload {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let mut entries: Vec<DataSegment> = Vec::new();
        for _ in 0..count {
            entries.push(DataSegment::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            entries: entries,
        })
    }
}

impl Sizeof for DataSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_count = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_entries: u32 = self.entries.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_entries
    }
}

impl DataSegment {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut index: u64 = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let offset = InitExpr::parse(reader)?;
        let mut size: u64 = 0;
        match read_unsigned_leb128(reader, &mut size) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let mut data: Vec<u8> = Vec::new();
        for _ in 0..size {
            match read_8(reader) {
                Ok(d) => data.push(d[0]),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            };
        }

        Ok(Self {
            index: index as VarUInt32,
            offset: offset,
            size: size as VarUInt32,
            data: data,
        })
    }

    /// 対応する線形メモリのインデックスを返す
    ///
    /// Wasm v1のMVPモデルでは、1つで固定
    pub fn get_memory_index(&self) -> u32 {
        self.index
    }

    /// (not implemented) オフセットを返す(予定)
    pub fn get_offset(&self) -> i32 {
        panic!("not implemented")
    }

    /// dataのサイズをを返す
    pub fn get_data_size(&self) -> u32 {
        self.size
    }

    /// dataの実体を返す
    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl Sizeof for DataSegment {
    fn sizeof(&self) -> u32 {
        let sizeof_index = usage_bytes_leb128_u(self.index as u64) as u32;
        let sizeof_offset = self.offset.sizeof();
        let sizeof_size = usage_bytes_leb128_u(self.size as u64) as u32;
        let sizeof_data = self.data.len() as u32;

        sizeof_index + sizeof_offset + sizeof_size + sizeof_data
    }
}
