use std::io::Read;

use crate::readers::usage_bytes_leb128_u;
use crate::readers::{read_8, read_signed_leb128, read_unsigned_leb128};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::code::Expr;
use crate::wasm_components::sections::ParseError;
use crate::wasm_components::types::number_types::*;

/*
 * Language types
 *
 * Wasm/Watのプログラムで利用されるデータ型(i32, i64, f32, f64, ...etc)
 * number_typesとは異なることに注意
 */

#[derive(Debug)]
pub enum LangTypes {
    I32,     // 0x7f
    I64,     // 0x7e
    F32,     // 0x7d
    F64,     // 0x7c
    ANYFUNC, // 0x70
    FUNC,    // 0x60
    PSEUDO,  // 0x40
}

impl LangTypes {
    pub fn convert_from_vint7(v: VarInt7) -> Result<Self, ParseError> {
        match v {
            // 0x7f => LangTypes::I32,
            // 0x7e => LangTypes::I64,
            // 0x7d => LangTypes::F32,
            // 0x7c => LangTypes::F64,
            // 0x70 => LangTypes::ANYFUNC,
            // 0x60 => LangTypes::FUNC,
            // 0x40 => LangTypes::PSEUDO,
            -0x01 => Ok(LangTypes::I32),
            -0x02 => Ok(LangTypes::I64),
            -0x03 => Ok(LangTypes::F32),
            -0x04 => Ok(LangTypes::F64),
            -0x10 => Ok(LangTypes::ANYFUNC),
            -0x20 => Ok(LangTypes::FUNC),
            -0x40 => Ok(LangTypes::PSEUDO),
            _ => Err(ParseError::FormatError(format!("unknown type: v={}", v))), // panic!("unknown type: v={}", v),
        }
    }
}

// pub type ValueType = LangTypes;
#[derive(Debug)]
pub struct ValueType {
    pub value: LangTypes,
}
impl ValueType {
    pub fn new(v: VarInt7) -> Result<Self, ParseError> {
        let vt = LangTypes::convert_from_vint7(v)?;
        match vt {
            LangTypes::ANYFUNC | LangTypes::FUNC | LangTypes::PSEUDO => {
                // panic!("{:?} is not value type", vt)
                Err(ParseError::FormatError(format!(
                    "{:?} is not value type",
                    vt
                )))
            }
            _ => Ok(Self { value: vt }),
        }
    }

    // BlockTypeやElemTypeと全く同じ処理になっている。
    // なんとかまとめられないか？
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut v: i64 = 0;
        match read_signed_leb128(reader, &mut v) {
            Ok(_) => Ok(Self::new(v as VarInt7)?),
            Err(err) => Err(ParseError::ReaderError(format!("{:?}", err))),
        }
    }
}

impl Sizeof for ValueType {
    fn sizeof(&self) -> u32 {
        1
    }
}

// pub type BlockType = LangTypes;
#[derive(Debug)]
pub struct BlockType {
    pub value: LangTypes,
}
impl BlockType {
    pub fn new(v: VarInt7) -> Result<Self, ParseError> {
        let vt = LangTypes::convert_from_vint7(v)?;
        match vt {
            LangTypes::ANYFUNC | LangTypes::FUNC => Err(ParseError::FormatError(format!(
                "{:?} is not block type",
                vt
            ))),
            _ => Ok(Self { value: vt }),
        }
    }

    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut v: i64 = 0;
        match read_signed_leb128(reader, &mut v) {
            Ok(_) => Ok(Self::new(v as VarInt7)?),
            Err(err) => Err(ParseError::ReaderError(format!("{:?}", err))),
        }
    }
}

impl Sizeof for BlockType {
    fn sizeof(&self) -> u32 {
        1
    }
}

// pub type ElemType = LangTypes;
#[derive(Debug)]
pub struct ElemType {
    pub value: LangTypes,
}
impl ElemType {
    pub fn new(v: VarInt7) -> Result<Self, ParseError> {
        let vt = LangTypes::convert_from_vint7(v)?;
        match vt {
            LangTypes::ANYFUNC => Ok(Self { value: vt }),
            _ => Err(ParseError::FormatError(format!(
                "{:?} is not elem type",
                vt
            ))),
        }
    }

    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut v: i64 = 0;
        match read_signed_leb128(reader, &mut v) {
            Ok(_) => Ok(Self::new(v as VarInt7)?),
            Err(err) => Err(ParseError::ReaderError(format!("{:?}", err))),
        }
    }
}

impl Sizeof for ElemType {
    fn sizeof(&self) -> u32 {
        1
    }
}

#[derive(Debug)]
pub struct FuncType {
    pub form: VarInt7,
    pub param_count: VarUInt32,
    pub param_types: Vec<ValueType>,
    pub return_count: VarUInt1,
    pub return_type: Option<ValueType>, // return_type: Vec<ValueType>
}

impl FuncType {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut form = 0;
        match read_signed_leb128(reader, &mut form) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut param_count = 0; // VarUInt32
        match read_unsigned_leb128(reader, &mut param_count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut param_types: Vec<ValueType> = Vec::new();
        for _ in 0..param_count {
            param_types.push(ValueType::parse(reader)?);
        }

        let mut return_count = 0; // VarUInt1
        match read_unsigned_leb128(reader, &mut return_count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let return_type: Option<ValueType>;
        if return_count == 1 {
            return_type = Some(ValueType::parse(reader)?);
        } else {
            return_type = None;
        }

        Ok(Self {
            form: form as VarInt7,
            param_count: param_count as VarUInt32,
            param_types: param_types,
            return_count: return_count as VarUInt1,
            return_type: return_type,
        })
    }
}

impl Sizeof for FuncType {
    fn sizeof(&self) -> u32 {
        let sizeof_form: u32 = 1;
        let sizeof_param_count: u32 = usage_bytes_leb128_u(self.param_count as u64) as u32;
        let sizeof_param_types: u32 = self.param_types.iter().map(|x| x.sizeof()).sum();
        let sizeof_return_count: u32 = 1;
        let sizeof_return_type: u32 = if self.return_type.is_some() {
            self.return_type.as_ref().unwrap().sizeof()
        } else {
            0
        };

        sizeof_form
            + sizeof_param_count
            + sizeof_param_types
            + sizeof_return_count
            + sizeof_return_type
    }
}

#[derive(Debug)]
pub struct GlobalType {
    pub content_type: ValueType,
    pub mutability: VarUInt1,
}
impl GlobalType {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let content_type = ValueType::parse(reader)?;
        // 0 if immutable, 1 if mutable
        let mut mutability = 0; // VarUInt1
        match read_unsigned_leb128(reader, &mut mutability) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        Ok(Self {
            content_type: content_type,
            mutability: mutability as VarUInt1,
        })
    }
}

impl Sizeof for GlobalType {
    fn sizeof(&self) -> u32 {
        let sizeof_content_type = self.content_type.sizeof();
        let sizeof_mutability: u32 = 1;

        sizeof_content_type + sizeof_mutability
    }
}

#[derive(Debug)]
pub struct TableType {
    pub element_type: ElemType,
    pub limits: ResizableLimits,
}
impl TableType {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let elem_type = ElemType::parse(reader)?;
        let limits = ResizableLimits::parse(reader)?;

        Ok(Self {
            element_type: elem_type,
            limits: limits,
        })
    }
}

impl Sizeof for TableType {
    fn sizeof(&self) -> u32 {
        let sizeof_element_type = self.element_type.sizeof();
        let sizeof_limits = self.limits.sizeof();

        sizeof_element_type + sizeof_limits
    }
}

#[derive(Debug)]
pub struct MemoryType {
    pub limits: ResizableLimits,
}
impl MemoryType {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let limits = ResizableLimits::parse(reader)?;
        Ok(Self { limits: limits })
    }
}

impl Sizeof for MemoryType {
    fn sizeof(&self) -> u32 {
        self.limits.sizeof()
    }
}

#[derive(Debug)]
pub struct ResizableLimits {
    pub flags: VarUInt1,
    pub initial: VarUInt32,
    pub maximum: Option<VarUInt32>,
}
impl ResizableLimits {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let mut flags = 0; // VarUInt1
        match read_unsigned_leb128(reader, &mut flags) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut initial = 0; // VarUInt32
        match read_unsigned_leb128(reader, &mut initial) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let maximum: Option<VarUInt32>;
        if flags == 1 {
            let mut m = 0; // Option<VarUInt32>
            match read_unsigned_leb128(reader, &mut m) {
                Ok(_rs) => (/* To check read size */),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            }
            maximum = Some(m as VarUInt32);
        } else {
            maximum = None;
        }

        Ok(Self {
            flags: flags as VarUInt1,
            initial: initial as VarUInt32,
            maximum: maximum,
        })
    }
}

impl Sizeof for ResizableLimits {
    fn sizeof(&self) -> u32 {
        let sizeof_flags: u32 = 1;
        let sizeof_initial: u32 = usage_bytes_leb128_u(self.initial as u64) as u32;
        let sizeof_maximum: u32 = if self.flags == 1 {
            usage_bytes_leb128_u(self.maximum.unwrap() as u64) as u32
        } else {
            0
        };

        sizeof_flags + sizeof_initial + sizeof_maximum
    }
}

// Single byte
#[derive(Debug)]
pub enum ExternalKind {
    Function,
    Table,
    Memory,
    Global,
}

impl ExternalKind {
    pub fn parse<R: Read>(reader: &mut R) -> Result<Self, ParseError> {
        let kind_head = match read_8(reader) {
            Ok(kh) => kh[0],
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };
        Ok(match kind_head {
            0 => ExternalKind::Function,
            1 => ExternalKind::Table,
            2 => ExternalKind::Memory,
            3 => ExternalKind::Global,
            _ => panic!("unknown external kind: kind_head={}", kind_head),
        })
    }
}

impl Sizeof for ExternalKind {
    fn sizeof(&self) -> u32 {
        1
    }
}

// impl ExternalKind {
//     pub fn new(v: u8) -> ExternalKind {
//         match v {
//             0 => ExternalKind::Function,
//             1 => ExternalKind::Table,
//             2 => ExternalKind::Memory,
//             3 => ExternalKind::Global,
//             _ => panic!("unknown external kind: v={}", v),
//         }
//     }
// }

pub type InitExpr = Expr;
