use std::io::{Read, Seek};

use crate::parser::Parser;
use crate::wasm_components::base::Sizeof;
use crate::wasm_components::sections::*;

// #[derive(Debug)]
pub struct WasmModule {
    pub magic_and_version: MagicAndVersion,
    pub type_section: Option<TypeSection>,
    pub import_section: Option<ImportSection>,
    pub function_section: Option<FunctionSection>,
    pub table_section: Option<TableSection>,
    pub memory_section: Option<MemorySection>,
    pub global_section: Option<GlobalSection>,
    pub export_section: Option<ExportSection>,
    pub start_section: Option<StartSection>,
    pub element_section: Option<ElementSection>,
    pub code_section: Option<CodeSection>,
    pub data_section: Option<DataSection>,
    pub custom_sections: Vec<CustomSection>,
}

impl WasmModule {
    pub fn parse<R: Read + Seek>(reader: &mut R) -> Result<Self, ParseError> {
        let mut parser = Parser::new(reader);
        Ok(parser.parse_all()?)
    }

    pub fn empty(mv: &MagicAndVersion) -> Self {
        Self {
            magic_and_version: mv.clone(),
            type_section: None,
            import_section: None,
            function_section: None,
            table_section: None,
            memory_section: None,
            global_section: None,
            export_section: None,
            start_section: None,
            element_section: None,
            code_section: None,
            data_section: None,
            custom_sections: Vec::new(),
        }
    }
}

impl Sizeof for WasmModule {
    fn sizeof(&self) -> u32 {
        let sizeof_custom_sections: u32 = self
            .custom_sections
            .iter()
            .map(|sec| -> u32 { sec.sizeof() })
            .sum();

        self.magic_and_version.sizeof()
            + sizeof_option_section(&self.type_section)
            + sizeof_option_section(&self.import_section)
            + sizeof_option_section(&self.function_section)
            + sizeof_option_section(&self.table_section)
            + sizeof_option_section(&self.memory_section)
            + sizeof_option_section(&self.global_section)
            + sizeof_option_section(&self.export_section)
            + sizeof_option_section(&self.start_section)
            + sizeof_option_section(&self.element_section)
            + sizeof_option_section(&self.code_section)
            + sizeof_option_section(&self.data_section)
            + sizeof_custom_sections
    }
}

fn sizeof_option_section<T: Sizeof>(section: &Option<T>) -> u32 {
    if section.is_some() {
        section.as_ref().unwrap().sizeof()
    } else {
        0
    }
}
