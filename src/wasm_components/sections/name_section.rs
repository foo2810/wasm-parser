use std::io::{Read, Seek, SeekFrom};
use std::str;

use super::base::ParseError;

use crate::readers::{read_unsigned_leb128, read_x, usage_bytes_leb128_u};
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::types::{VarUInt32, VarUInt7};

#[derive(Debug)]
pub struct NameSectionPayload {
    // Name Subsections
    module_name: Option<ModuleName>,
    function_names: Option<FunctionNames>,
    local_names: Option<LocalNames>,
}

#[derive(Debug)]
pub struct ModuleName {
    name_type: VarUInt7,
    name_payload_len: VarUInt32,
    name_len: VarUInt32,
    name_str: String,
}

#[derive(Debug)]
pub struct FunctionNames {
    name_type: VarUInt7,
    name_payload_len: VarUInt32,
    func_map: NameMap,
}

#[derive(Debug)]
pub struct LocalNames {
    name_type: VarUInt7,
    name_payload_len: VarUInt32,
    count: VarUInt32,
    funcs: Vec<LocalName>,
}

#[derive(Debug)]
pub struct LocalName {
    index: VarUInt32,
    local_map: NameMap,
}

#[derive(Debug)]
pub struct NameMap {
    count: VarUInt32,
    names: Vec<Naming>,
}

#[derive(Debug)]
pub struct Naming {
    index: VarUInt32,
    name_len: VarUInt32,
    name_str: String,
}

impl NameSectionPayload {
    pub fn parse<R: Read + Seek>(reader: &mut R, payload_size: u32) -> Result<Self, ParseError> {
        let mut read_size: u32 = 0;

        let mut module_name: Option<ModuleName> = None;
        let mut function_names: Option<FunctionNames> = None;
        let mut local_names: Option<LocalNames> = None;

        while read_size < payload_size {
            let mut name_type: u64 = 0;
            let sizeof_name_type: u32;
            match read_unsigned_leb128(reader, &mut name_type) {
                Ok(rs) => (sizeof_name_type = rs as u32),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            };

            let mut name_payload_len: u64 = 0;
            let sizeof_name_payload_len: u32;
            match read_unsigned_leb128(reader, &mut name_payload_len) {
                Ok(rs) => (sizeof_name_payload_len = rs as u32),
                Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
            };

            match name_type {
                0 => {
                    module_name = Some(ModuleName::parse(
                        reader,
                        name_type as VarUInt7,
                        name_payload_len as VarUInt32,
                    )?);
                    read_size += module_name.as_ref().unwrap().sizeof()
                }
                1 => {
                    function_names = Some(FunctionNames::parse(
                        reader,
                        name_type as VarUInt7,
                        name_payload_len as VarUInt32,
                    )?);
                    read_size += function_names.as_ref().unwrap().sizeof();
                }
                2 => {
                    local_names = Some(LocalNames::parse(
                        reader,
                        name_type as VarUInt7,
                        name_payload_len as VarUInt32,
                    )?);
                    read_size += local_names.as_ref().unwrap().sizeof();
                }
                _ => {
                    println!(" > Warn: unexpected: {}", name_type);
                    let sizeof_unknown_subsection =
                        sizeof_name_type + sizeof_name_payload_len + name_payload_len as u32;
                    read_size += sizeof_unknown_subsection;
                    match reader.seek(SeekFrom::Current(name_payload_len as i64)) {
                        Ok(_rs) => (/* To check read size */),
                        Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
                    };
                }
            };
        }

        if read_size != payload_size {
            return Err(ParseError::FormatError(String::from(format!(
                "payload_size and read_size are not same: payload_size={}, read_size={}",
                payload_size, read_size
            ))));
        }

        Ok(Self {
            module_name: module_name,
            function_names: function_names,
            local_names: local_names,
        })
    }

    pub fn get_module_name(&self) -> Option<&ModuleName> {
        self.module_name.as_ref()
    }

    pub fn get_function_names(&self) -> Option<&FunctionNames> {
        self.function_names.as_ref()
    }

    pub fn get_local_names(&self) -> Option<&LocalNames> {
        self.local_names.as_ref()
    }
}

impl Sizeof for NameSectionPayload {
    fn sizeof(&self) -> u32 {
        let sizeof_module_name = self
            .module_name
            .as_ref()
            .map(|mod_name| mod_name.sizeof())
            .unwrap_or(0);
        let sizeof_func_name = self
            .function_names
            .as_ref()
            .map(|mod_name| mod_name.sizeof())
            .unwrap_or(0);
        let sizeof_local_name = self
            .local_names
            .as_ref()
            .map(|mod_name| mod_name.sizeof())
            .unwrap_or(0);

        sizeof_module_name + sizeof_func_name + sizeof_local_name
    }
}

impl ModuleName {
    pub fn parse<R: Read + Seek>(
        reader: &mut R,
        name_type: VarUInt7,
        name_payload_len: VarUInt32,
    ) -> Result<Self, ParseError> {
        let mut name_len: u64 = 0;
        match read_unsigned_leb128(reader, &mut name_len) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let name_str: String;
        match read_x(reader, name_len as usize) {
            Ok(data) => match str::from_utf8(&data) {
                Ok(s) => name_str = String::from(s),
                Err(err) => return Err(ParseError::FormatError(format!("{:?}", err))),
            },
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        Ok(Self {
            name_type: name_type as VarUInt7,
            name_payload_len: name_payload_len as VarUInt32,
            name_len: name_len as VarUInt32,
            name_str: name_str,
        })
    }

    pub fn get_name_type(&self) -> u8 {
        self.name_type as u8
    }

    pub fn get_payload_size(&self) -> u32 {
        self.name_payload_len as u32
    }

    pub fn get_name_length(&self) -> u32 {
        self.name_len as u32
    }

    pub fn get_name(&self) -> &String {
        &self.name_str
    }
}

impl Sizeof for ModuleName {
    fn sizeof(&self) -> u32 {
        let sizeof_name_type = usage_bytes_leb128_u(self.name_type as u64) as u32;
        let sizeof_name_payload_len = usage_bytes_leb128_u(self.name_payload_len as u64) as u32;
        let sizeof_name_len = usage_bytes_leb128_u(self.name_len as u64) as u32;
        let sizeof_name_str = self.name_len as u32;

        sizeof_name_type + sizeof_name_payload_len + sizeof_name_len + sizeof_name_str
    }
}

impl FunctionNames {
    pub fn parse<R: Read + Seek>(
        reader: &mut R,
        name_type: VarUInt7,
        name_payload_len: VarUInt32,
    ) -> Result<Self, ParseError> {
        let func_map = NameMap::parse(reader)?;

        Ok(Self {
            name_type: name_type as VarUInt7,
            name_payload_len: name_payload_len as VarUInt32,
            func_map: func_map,
        })
    }

    pub fn get_name_type(&self) -> u8 {
        self.name_type as u8
    }

    pub fn get_payload_size(&self) -> u32 {
        self.name_payload_len as u32
    }

    pub fn get_function_map(&self) -> &NameMap {
        &self.func_map
    }
}

impl Sizeof for FunctionNames {
    fn sizeof(&self) -> u32 {
        let sizeof_name_type = usage_bytes_leb128_u(self.name_type as u64) as u32;
        let sizeof_name_payload_len = usage_bytes_leb128_u(self.name_payload_len as u64) as u32;
        let sizeof_func_map = self.func_map.sizeof();

        sizeof_name_type + sizeof_name_payload_len + sizeof_func_map
    }
}

impl LocalNames {
    pub fn parse<R: Read + Seek>(
        reader: &mut R,
        name_type: VarUInt7,
        name_payload_len: VarUInt32,
    ) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut funcs: Vec<LocalName> = Vec::new();
        for _ in 0..count {
            funcs.push(LocalName::parse(reader)?);
        }

        Ok(Self {
            name_type: name_type as VarUInt7,
            name_payload_len: name_payload_len as VarUInt32,
            count: count as VarUInt32,
            funcs: funcs,
        })
    }

    pub fn get_name_type(&self) -> u8 {
        self.name_type as u8
    }

    pub fn get_payload_size(&self) -> u32 {
        self.name_payload_len as u32
    }

    pub fn get_count(&self) -> u32 {
        self.count as u32
    }

    pub fn get_locals(&self) -> Vec<&LocalName> {
        self.funcs.iter().collect()
    }
}

impl Sizeof for LocalNames {
    fn sizeof(&self) -> u32 {
        let sizeof_name_type = usage_bytes_leb128_u(self.name_type as u64) as u32;
        let sizeof_name_payload_len = usage_bytes_leb128_u(self.name_payload_len as u64) as u32;
        let sizeof_count: u32 = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_funcs: u32 = self.funcs.iter().map(|x| x.sizeof()).sum();

        sizeof_name_type + sizeof_name_payload_len + sizeof_count + sizeof_funcs
    }
}

impl LocalName {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let mut index: u64 = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let local_map = NameMap::parse(reader)?;

        Ok(Self {
            index: index as VarUInt32,
            local_map: local_map,
        })
    }

    /// func_idxを返す
    pub fn get_indice(&self) -> u32 {
        self.index as u32
    }

    /// 対応するローカルスコープのすべてのローカル変数情報を返す
    pub fn get_local_map(&self) -> &NameMap {
        &self.local_map
    }
}

impl Sizeof for LocalName {
    fn sizeof(&self) -> u32 {
        let sizeof_index = usage_bytes_leb128_u(self.index as u64) as u32;
        let sizeof_local_map = self.local_map.sizeof();

        sizeof_index + sizeof_local_map
    }
}

impl NameMap {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let mut count: u64 = 0;
        match read_unsigned_leb128(reader, &mut count) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut names: Vec<Naming> = Vec::new();
        for _ in 0..count {
            names.push(Naming::parse(reader)?);
        }

        Ok(Self {
            count: count as VarUInt32,
            names: names,
        })
    }

    pub fn get_num_names(&self) -> u32 {
        self.count as u32
    }

    pub fn get_name_list(&self) -> Vec<&Naming> {
        self.names.iter().collect()
    }
}

impl Sizeof for NameMap {
    fn sizeof(&self) -> u32 {
        let sizeof_count: u32 = usage_bytes_leb128_u(self.count as u64) as u32;
        let sizeof_names: u32 = self.names.iter().map(|x| x.sizeof()).sum();

        sizeof_count + sizeof_names
    }
}

impl Naming {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let mut index: u64 = 0;
        match read_unsigned_leb128(reader, &mut index) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let mut name_len: u64 = 0;
        match read_unsigned_leb128(reader, &mut name_len) {
            Ok(_rs) => (/* To check read size */),
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        let name_str: String;
        match read_x(reader, name_len as usize) {
            Ok(data) => match str::from_utf8(&data) {
                Ok(s) => name_str = String::from(s),
                Err(err) => return Err(ParseError::FormatError(format!("{:?}", err))),
            },
            Err(err) => return Err(ParseError::ReaderError(format!("{:?}", err))),
        };

        Ok(Self {
            index: index as VarUInt32,
            name_len: name_len as VarUInt32,
            name_str: name_str,
        })
    }

    pub fn get_indice(&self) -> u32 {
        self.index as u32
    }

    pub fn get_name_len(&self) -> u32 {
        self.name_len as u32
    }

    pub fn get_name_str(&self) -> &String {
        &self.name_str
    }
}

impl Sizeof for Naming {
    fn sizeof(&self) -> u32 {
        let sizeof_index = usage_bytes_leb128_u(self.index as u64) as u32;
        let sizeof_name_len = usage_bytes_leb128_u(self.name_len as u64) as u32;
        let sizeof_name_str = self.name_len as u32;

        sizeof_index + sizeof_name_len + sizeof_name_str
    }
}
