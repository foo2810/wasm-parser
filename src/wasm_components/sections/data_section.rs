use std::io::{Read, Seek};

use super::base::{ParseError, SectionCommon};

use crate::readers::{read_8, read_unsigned_leb128, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{InitExpr, VarUInt32};

#[derive(Debug)]
pub struct DataSection {
    pub common: SectionCommon,
    pub payload: DataSectionPayload,
}

#[derive(Debug)]
pub struct DataSectionPayload {
    pub count: VarUInt32,
    pub entries: Vec<DataSegment>,
}

#[derive(Debug)]
pub struct DataSegment {
    pub index: VarUInt32,
    pub offset: InitExpr,
    pub size: VarUInt32, // size of data (bytes)
    pub data: Vec<u8>,
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
}

impl Sizeof for DataSection {
    fn sizeof(&self) -> u32 {
        let sizeof_common = self.common.sizeof();
        let sizeof_payload = self.payload.sizeof();

        sizeof_common + sizeof_payload
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
