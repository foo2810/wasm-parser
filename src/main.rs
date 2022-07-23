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
    // Optional arg
    // #[clap(short = 'n', long = "name")]
    // name: Option<String>,

    // Required arg
    // #[clap(short = 'c', long = "count", default_value="Alice")]  // ここでもdefaultを設定できるっぽい:
    // #[clap(short = 'c', long = "count")]
    // count: i32,

    // Positional arg
    path: String,
}

fn main() {
    let args = CmdArgs::parse();

    // let path = Path::new("wasm_sample/sample.wasm");
    // let path = Path::new("wasm_sample/rust-wasm.wasm");
    let path = Path::new(args.path.as_str());

    // File open
    // let file = match File::open(path) {
    //     Ok(file) => file,
    //     Err(err) => panic!("could not open {}: {}", display, err),
    // };
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

    // printer::print_all_section_for_debug(&wasm_module);
}
