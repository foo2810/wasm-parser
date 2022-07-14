use std::io::{BufReader, Read};

use crate::readers::read_8;
use crate::wasm_components::code::Expr;
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
    pub fn convert_from_vint7(v: VarInt7) -> Self {
        match v {
            // 0x7f => LangTypes::I32,
            // 0x7e => LangTypes::I64,
            // 0x7d => LangTypes::F32,
            // 0x7c => LangTypes::F64,
            // 0x70 => LangTypes::ANYFUNC,
            // 0x60 => LangTypes::FUNC,
            // 0x40 => LangTypes::PSEUDO,
            -0x01 => LangTypes::I32,
            -0x02 => LangTypes::I64,
            -0x03 => LangTypes::F32,
            -0x04 => LangTypes::F64,
            -0x10 => LangTypes::ANYFUNC,
            -0x20 => LangTypes::FUNC,
            -0x40 => LangTypes::PSEUDO,
            _ => panic!("unknown type: v={}", v),
        }
    }
}

// pub type ValueType = LangTypes;
#[derive(Debug)]
pub struct ValueType {
    pub value: LangTypes,
}
impl ValueType {
    pub fn new(v: VarInt7) -> Self {
        let vt = LangTypes::convert_from_vint7(v);
        match vt {
            LangTypes::ANYFUNC | LangTypes::FUNC | LangTypes::PSEUDO => {
                panic!("{:?} is not value type", vt)
            }
            _ => Self { value: vt },
        }
    }

    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let v = leb128::read::signed(reader).unwrap() as VarInt7;
        Self::new(v)
    }
}

// pub type BlockType = LangTypes;
#[derive(Debug)]
pub struct BlockType {
    pub value: LangTypes,
}
impl BlockType {
    pub fn new(v: VarInt7) -> Self {
        let vt = LangTypes::convert_from_vint7(v);
        match vt {
            LangTypes::ANYFUNC | LangTypes::FUNC => {
                panic!("{:?} is not block type", vt)
            }
            _ => Self { value: vt },
        }
    }

    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let v = leb128::read::signed(reader).unwrap() as VarInt7;
        Self::new(v)
    }
}

// pub type ElemType = LangTypes;
#[derive(Debug)]
pub struct ElemType {
    pub value: LangTypes,
}
impl ElemType {
    pub fn new(v: VarInt7) -> Self {
        let vt = LangTypes::convert_from_vint7(v);
        match vt {
            LangTypes::ANYFUNC => Self { value: vt },
            _ => {
                panic!("{:?} is not elem type", vt)
            }
        }
    }

    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let v = leb128::read::signed(reader).unwrap() as VarInt7;
        Self::new(v)
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
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let form = leb128::read::signed(reader).unwrap() as VarInt7;
        let param_count = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let mut param_types: Vec<ValueType> = Vec::new();
        for _ in 0..param_count {
            param_types.push(ValueType::parse(reader));
        }
        let return_count = leb128::read::unsigned(reader).unwrap() as VarUInt1;
        let return_type: Option<ValueType>;
        if return_count == 1 {
            return_type = Some(ValueType::parse(reader));
        } else {
            return_type = None;
        }

        Self {
            form: form,
            param_count: param_count,
            param_types: param_types,
            return_count: return_count,
            return_type: return_type,
        }
    }
}

#[derive(Debug)]
pub struct GlobalType {
    pub content_type: ValueType,
    pub mutability: VarUInt1,
}
impl GlobalType {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let content_type = ValueType::parse(reader);
        // 0 if immutable, 1 if mutable
        let mutability = leb128::read::unsigned(reader).unwrap() as VarUInt1;

        Self {
            content_type: content_type,
            mutability: mutability,
        }
    }
}

#[derive(Debug)]
pub struct TableType {
    pub element_type: ElemType,
    pub limits: ResizableLimits,
}
impl TableType {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let elem_type = ElemType::parse(reader);
        let limits = ResizableLimits::parse(reader);

        Self {
            element_type: elem_type,
            limits: limits,
        }
    }
}

#[derive(Debug)]
pub struct MemoryType {
    pub limits: ResizableLimits,
}
impl MemoryType {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let limits = ResizableLimits::parse(reader);
        Self { limits: limits }
    }
}

#[derive(Debug)]
pub struct ResizableLimits {
    pub flags: VarUInt1,
    pub initial: VarUInt32,
    pub maximum: Option<VarUInt32>,
}
impl ResizableLimits {
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let flags = leb128::read::unsigned(reader).unwrap() as VarUInt1;
        let initial = leb128::read::unsigned(reader).unwrap() as VarUInt32;
        let maximum: Option<VarUInt32>;
        if flags == 1 {
            maximum = Some(leb128::read::unsigned(reader).unwrap() as VarUInt32);
        } else {
            maximum = None;
        }

        Self {
            flags: flags,
            initial: initial,
            maximum: maximum,
        }
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
    pub fn parse<R: Read>(reader: &mut BufReader<R>) -> Self {
        let kind_head = read_8(reader)[0];
        match kind_head {
            0 => ExternalKind::Function,
            1 => ExternalKind::Table,
            2 => ExternalKind::Memory,
            3 => ExternalKind::Global,
            _ => panic!("unknown external kind: kind_head={}", kind_head),
        }
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
