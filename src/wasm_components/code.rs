use crate::readers::{read_8, read_x};
use std::io::{BufReader, Read, Seek, SeekFrom};

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
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let bytes = read_all_instrs(reader);
        Expr { bytes: bytes }
    }
}

impl FunctionBody {
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Self {
        // body_sizeはFunctionBodyのbody_sizeフィールドを除く部分のバイト数を表す
        let body_size = leb128::read::unsigned(reader).unwrap() as VarUInt32;

        let s_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        let local_count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let e_offset = reader.seek(SeekFrom::Current(0)).unwrap();

        // local_countの値が格納されているバイト数を計算する。
        // LEB128では値の大きさによって使用するバイト数が異なるため、
        // readerのカーソルの位置から強引に計算する。
        let sizeof_local_count = (e_offset - s_offset) as i64;

        let mut locals: Vec<LocalEntry> = Vec::new();
        let s_offset = reader.seek(SeekFrom::Current(0)).unwrap();
        for _ in 0..local_count {
            locals.push(LocalEntry::parse(reader));
        }
        let e_offset = reader.seek(SeekFrom::Current(0)).unwrap();

        // sizeof_local_countと同様
        let sizeof_locals = (e_offset - s_offset) as i64;

        // code_size = body_size - sizeof(local_count)- sizeof(locals) - sizeof(end)
        let code_size = (body_size as i64) - sizeof_local_count - sizeof_locals - 1;
        let code: Vec<u8> = read_x(reader, code_size as usize); // <- include 'end' instruction (Must not be included)
        let end = read_8(reader)[0]; // expected 0x0B

        // for debug
        if end != 0x0B {
            panic!("FunctionBody.end is invalid: {:?}", end);
        }

        Self {
            body_size: body_size,
            local_count: local_count,
            locals: locals,
            code: code,
            end: end,
        }
    }
}

impl LocalEntry {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let type_ = ValueType::parse(reader);
        Self {
            count: count,
            type_: type_,
        }
    }
}

fn read_all_instrs<R: Read>(reader: &mut BufReader<R>) -> Vec<u8> {
    let mut instrs: Vec<u8> = Vec::new();
    loop {
        let b = read_8(reader)[0];
        instrs.push(b);
        if b == 0x0B {
            break;
        }
    }
    instrs
}
