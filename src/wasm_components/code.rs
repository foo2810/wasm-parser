use std::io::{BufReader, Read, Seek, SeekFrom};

use crate::readers::{read_8, read_unsigned_leb128, read_x};
use crate::wasm_components::sections::ParseError;

use super::types::{ValueType, VarUInt32};

#[derive(Debug)]
pub enum InstrType {}

#[derive(Debug)]
pub struct Expr {
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct FunctionBody {
    pub body_size: VarUInt32,
    pub local_count: VarUInt32,
    pub locals: Vec<LocalEntry>,
    pub code: Vec<u8>,
    pub end: u8, // 0x0B = 'end' instruction
}

#[derive(Debug)]
pub struct LocalEntry {
    pub count: VarUInt32,
    pub type_: ValueType,
}

impl Expr {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        let bytes = read_all_instrs(reader)?;
        Ok(Expr { bytes: bytes })
    }
}

impl FunctionBody {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // body_sizeはFunctionBodyのbody_sizeフィールドを除く部分のバイト数を表す
        let mut body_size: u64 = 0; // VarUInt32
        match read_unsigned_leb128(reader, &mut body_size) {
            Ok(rs) => (/* To check read size */),
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
        let s_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        for _ in 0..local_count {
            locals.push(LocalEntry::parse(reader)?);
        }
        let e_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        let sizeof_locals = (e_offset - s_offset) as i64; // sizeof_local_countと同様

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
}

impl LocalEntry {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
        // let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut count = 0; // VarUInt32
        match read_unsigned_leb128(reader, &mut count) {
            Ok(rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        let type_ = ValueType::parse(reader)?;

        Ok(Self {
            count: count as VarUInt32,
            type_: type_,
        })
    }
}

fn read_all_instrs<R: Read>(reader: &mut BufReader<R>) -> Result<Vec<u8>, ParseError> {
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
