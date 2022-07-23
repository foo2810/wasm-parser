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

    pub fn get_magic_and_version(&self) -> &MagicAndVersion {
        &self.magic_and_version
    }

    pub fn get_type_section(&self) -> Option<&TypeSection> {
        self.type_section.as_ref()
    }

    pub fn get_import_section(&self) -> Option<&ImportSection> {
        self.import_section.as_ref()
    }

    pub fn get_function_section(&self) -> Option<&FunctionSection> {
        self.function_section.as_ref()
    }

    pub fn get_table_section(&self) -> Option<&TableSection> {
        self.table_section.as_ref()
    }

    pub fn get_memory_section(&self) -> Option<&MemorySection> {
        self.memory_section.as_ref()
    }

    pub fn get_global_section(&self) -> Option<&GlobalSection> {
        self.global_section.as_ref()
    }

    pub fn get_export_section(&self) -> Option<&ExportSection> {
        self.export_section.as_ref()
    }

    pub fn get_start_section(&self) -> Option<&StartSection> {
        self.start_section.as_ref()
    }

    pub fn get_element_section(&self) -> Option<&ElementSection> {
        self.element_section.as_ref()
    }

    pub fn get_code_section(&self) -> Option<&CodeSection> {
        self.code_section.as_ref()
    }

    pub fn get_data_section(&self) -> Option<&DataSection> {
        self.data_section.as_ref()
    }

    pub fn get_custom_sections(&self) -> Vec<&CustomSection> {
        self.custom_sections.iter().collect()
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
