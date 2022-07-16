use std::io::{BufReader, Read, Seek};

use crate::parser::Parser;
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
    pub fn parse<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, ParseError> {
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
