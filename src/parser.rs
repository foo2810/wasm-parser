use std::io::{Read, Seek};

use crate::readers::peep_8;
use crate::wasm_components::module::WasmModule;
use crate::wasm_components::sections::MagicAndVersion;
use crate::wasm_components::sections::*;

#[derive(Debug)]
pub struct Parser<'a, R: Read> {
    reader: &'a mut R,
    // offset: u64,     // offsetはreaderから取得する
}

// 構造体のメンバに参照を使う場合ライフタイム注釈が必要(その参照と構造体自身の生存期間の関係を明示するため)
impl<'a, R: Read + Seek> Parser<'a, R> {
    pub fn new(reader: &'a mut R) -> Parser<R> {
        Parser { reader: reader }
    }

    // readerを使って、バイナリを順に読んでいき、読み込んだ値をデータ構造に落とし込む
    pub fn parse_all(&mut self) -> Result<WasmModule, ParseError> {
        // Read magic(4 bytes) and version(4 bytes)
        let magic_and_version = MagicAndVersion::parse(self.reader)?;
        let mut module = WasmModule::empty(&magic_and_version);

        // Read body
        loop {
            let section_id = match peep_8(self.reader) {
                Ok(section_id) => section_id[0],
                Err(_) => {
                    // println!(" > Info : reach EOF");
                    break;
                }
            };

            match section_id {
                1 => {
                    let type_section = TypeSection::parse(self.reader)?;
                    module.type_section = Some(type_section);
                }
                2 => {
                    let import_section = ImportSection::parse(self.reader)?;
                    module.import_section = Some(import_section);
                }
                3 => {
                    let function_section = FunctionSection::parse(self.reader)?;
                    module.function_section = Some(function_section);
                }
                4 => {
                    let table_section = TableSection::parse(self.reader)?;
                    module.table_section = Some(table_section);
                }
                5 => {
                    let memory_section = MemorySection::parse(self.reader)?;
                    module.memory_section = Some(memory_section);
                }
                6 => {
                    let global_section = GlobalSection::parse(self.reader)?;
                    module.global_section = Some(global_section);
                }
                7 => {
                    let export_section = ExportSection::parse(self.reader)?;
                    module.export_section = Some(export_section);
                }
                8 => {
                    let start_section = StartSection::parse(self.reader)?;
                    module.start_section = Some(start_section);
                }
                9 => {
                    let element_section = ElementSection::parse(self.reader)?;
                    module.element_section = Some(element_section);
                }
                10 => {
                    let code_section = CodeSection::parse(self.reader)?;
                    module.code_section = Some(code_section);
                }
                11 => {
                    let data_section = DataSection::parse(self.reader)?;
                    module.data_section = Some(data_section);
                }
                0 => {
                    let custom_section = CustomSection::parse(self.reader)?;
                    module.custom_sections.push(custom_section);
                }
                _ => {
                    println!(
                        " > Info: section_id={}, unknow or not implemented",
                        section_id
                    );
                }
            }
        }
        Ok(module)
    }
}
