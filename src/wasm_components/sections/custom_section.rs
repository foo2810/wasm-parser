use std::io::{BufReader, Read, Seek, SeekFrom};
use std::str;

use crate::readers::read_x;
use crate::wasm_components::types::{VarUInt32, VarUInt7};

#[derive(Debug)]
pub struct CustomSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: u32,
    pub name: String,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct NameSection {
    pub id: u8,
    pub payload_len: u32,
    pub name_len: u32,
    pub name: String,
    pub payload: NameSectionPayload,
}

#[derive(Debug)]
pub struct NameSectionPayload {
    pub name: VarUInt7,
    pub name_payload_len: VarUInt32,
    pub name_payload_data: Vec<u8>, // Parse for name_payload_data is not implementd yet
}

impl CustomSection {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        let id = leb128::read::unsigned(reader).unwrap() as u8;
        if id != 0 {
            panic!("This Section is not CustomSection");
        }
        let payload_len = leb128::read::unsigned(reader).unwrap() as u32;

        let s_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        let nl = leb128::read::unsigned(reader).unwrap() as usize;
        let e_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        let sizeof_name_len = (e_offset - s_offset) as i64;

        let name_len = nl as u32;

        let s_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        let name = String::from(str::from_utf8(&read_x(reader, nl)).unwrap());
        let e_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        let sizeof_name = (e_offset - s_offset) as i64;

        let payload_size = payload_len as i64 - sizeof_name - sizeof_name_len;
        let payload = read_x(reader, payload_size as usize);

        Self {
            id: id,
            payload_len: payload_len,
            name_len: name_len,
            name: name,
            payload: payload,
        }
    }
}
