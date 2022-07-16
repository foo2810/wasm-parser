/*
 * WASMDUMP V0.1
 *
 * WASM_VERSION = 1.1
 */

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

    let _ = match parser.parse_all() {
        Ok(module) => module,
        Err(err) => match err {
            ParseError::ReaderError(msg) => panic!(" > Error: {}", msg),
            ParseError::FormatError(msg) => panic!(" > Error: {}", msg),
            ParseError::UnexpectedError(msg) => panic!(" > Error: {}", msg),
        },
    };
}
