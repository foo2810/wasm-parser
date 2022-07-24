/*
 * WASMDUMP V0.1
 *
 * WASM_VERSION = 1.1
 */

mod printer;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use wasmdump::parser::Parser as WasmParser;
use wasmdump::wasm_components::module::WasmModule;
use wasmdump::wasm_components::sections::ParseError;

use clap::Parser;
#[derive(Parser)]
#[clap(
    name = "wasmdump",
    author = "hogedamari",
    version = "0.1",
    about = "Parse wasm binary"
)]
struct CmdArgs {
    // Subcommand
    #[clap(arg_enum, value_parser)]
    action: Action,

    // Positional arg
    path: String,
}

#[derive(clap::ArgEnum, Clone, Debug)]
enum Action {
    Print,
    Dump,
    DumpTmp,
}

fn main() {
    let args = CmdArgs::parse();

    let path = Path::new(args.path.as_str());

    let file = File::open(path).unwrap();

    let mut reader = BufReader::new(file);

    let mut parser = WasmParser::new(&mut reader);

    let wasm_module = match parser.parse_all() {
        Ok(module) => module,
        Err(err) => match err {
            ParseError::ReaderError(msg) => panic!(" > Error: {}", msg),
            ParseError::FormatError(msg) => panic!(" > Error: {}", msg),
            ParseError::UnexpectedError(msg) => panic!(" > Error: {}", msg),
        },
    };

    match args.action {
        Action::Print => subcommand_print(&wasm_module),
        Action::Dump => panic!("not implemented !"),
        Action::DumpTmp => subcommand_dump(&wasm_module),
        // _ => panic!("unknown subcommand: {:?}", act),
    }
}

fn subcommand_print(wasm_module: &WasmModule) {
    printer::print_type_section(&wasm_module);
    printer::print_import_section(&wasm_module);
    printer::print_function_section(&wasm_module);
    printer::print_table_section(&wasm_module);
    printer::print_memory_section(&wasm_module);
    printer::print_global_section(&wasm_module);
    printer::print_export_section(&wasm_module);
    printer::print_start_section(&wasm_module);
    printer::print_element_section(&wasm_module);
    printer::print_data_section(&wasm_module);
    printer::print_custom_sections(&wasm_module);

    if false {
        printer::print_all_section_for_debug(&wasm_module);
    }
}

fn subcommand_dump(_wasm_module: &WasmModule) {
    println!("to be implemented")
}
