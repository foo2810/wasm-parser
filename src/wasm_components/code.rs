use std::io::{Read, Seek};

use crate::readers::usage_bytes_leb128_u;
use crate::readers::{read_8, read_unsigned_leb128, read_x};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::sections::ParseError;

use super::types::{LangTypes, ValueType, VarUInt32};

#[derive(Debug)]
pub enum InstrType {}

#[derive(Debug)]
pub struct Expr {
    bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct FunctionBody {
    body_size: VarUInt32,
    local_count: VarUInt32,
    locals: Vec<LocalEntry>,
    code: Vec<u8>,
    end: u8, // 0x0B = 'end' instruction
}

#[derive(Debug)]
pub struct LocalEntry {
    count: VarUInt32,
    type_: ValueType,
}

impl Expr {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let bytes = read_all_instrs(reader)?;
        Ok(Expr { bytes: bytes })
    }

    pub fn get_instrs(&self) -> &Vec<u8> {
        &self.bytes
    }
}

impl Sizeof for Expr {
    fn sizeof(&self) -> u32 {
        self.bytes.len() as u32
    }
}

impl FunctionBody {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        // body_sizeはFunctionBodyのbody_sizeフィールドを除く部分のバイト数を表す
        let mut body_size: u64 = 0; // VarUInt32
        match read_unsigned_leb128(reader, &mut body_size) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        // let s_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        // let local_count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut local_count: u64 = 0; // VarUInt32
        let sizeof_local_count: i64;
        match read_unsigned_leb128(reader, &mut local_count) {
            Ok(rs) => sizeof_local_count = rs as i64,
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        // let e_offset = reader.seek(SeekFrom::Current(0)).unwrap();

        // local_countの値が格納されているバイト数を計算する。
        // LEB128では値の大きさによって使用するバイト数が異なるため、
        // readerのカーソルの位置から強引に計算する。
        // let sizeof_local_count = (e_offset - s_offset) as i64;

        let mut locals: Vec<LocalEntry> = Vec::new();
        // let s_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        for _ in 0..local_count {
            locals.push(LocalEntry::parse(reader)?);
        }
        // let e_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        // let sizeof_locals = (e_offset - s_offset) as i64; // sizeof_local_countと同様
        let sizeof_locals: i64 = locals.iter().map(|x| -> i64 { x.sizeof() as i64 }).sum();

        // code_size = body_size - sizeof(local_count)- sizeof(locals) - sizeof(end)
        let code_size = (body_size as i64) - sizeof_local_count - sizeof_locals - 1;
        let code: Vec<u8> = match read_x(reader, code_size as usize) {
            Ok(data) => data,
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let end = match read_8(reader) {
            // expected 0x0B
            Ok(data) => data[0],
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        // for debug
        if end != 0x0B {
            // panic!("FunctionBody.end is invalid: {:?}", end);
            return Err(ParseError::FormatError(format!(
                "FunctionBody.end is invalid: {:?}",
                end
            )));
        }

        Ok(Self {
            body_size: body_size as VarUInt32,
            local_count: local_count as VarUInt32,
            locals: locals,
            code: code,
            end: end,
        })
    }

    pub fn get_locals(&self) -> Vec<&LangTypes> {
        self.locals.iter().map(|x| x.get_value_type()).collect()
    }
}

impl Sizeof for FunctionBody {
    fn sizeof(&self) -> u32 {
        let sizeof_body_size: u32 = usage_bytes_leb128_u(self.body_size as u64) as u32;
        let sizeof_local_count: u32 = usage_bytes_leb128_u(self.local_count as u64) as u32;
        let sizeof_locals: u32 = self.locals.iter().map(|x| x.sizeof()).sum();
        let sizeof_code: u32 = self.code.len() as u32;
        let sizeof_end: u32 = 1;

        sizeof_body_size + sizeof_local_count + sizeof_locals + sizeof_code + sizeof_end
    }
}

impl LocalEntry {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        // let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut count = 0; // VarUInt32
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let type_ = ValueType::parse(reader)?;

        Ok(Self {
            count: count as VarUInt32,
            type_: type_,
        })
    }

    pub fn get_value_type(&self) -> &LangTypes {
        self.type_.get_value()
    }
}

impl Sizeof for LocalEntry {
    fn sizeof(&self) -> u32 {
        let sizeof_count: u32 = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_type: u32 = self.type_.sizeof();

        sizeof_count + sizeof_type
    }
}

fn read_all_instrs<R: Read>(reader: &mut R) -> Result<Vec<u8>, ParseError> {
    let mut instrs: Vec<u8> = Vec::new();
    loop {
        let b = match read_8(reader) {
            Ok(data) => data[0],
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        instrs.push(b);
        if b == 0x0B {
            break;
        }
    }
    Ok(instrs)
}
