// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files
#[macro_use]
mod base;

mod code_section;
mod custom_section;
mod data_section;
mod element_section;
mod export_section;
mod function_section;
mod global_section;
mod import_section;
mod magic_and_version;
mod memory_section;
mod start_section;
mod table_section;
mod type_section;

pub use self::base::Section;
pub use self::base::{ParseError, SectionCommonInterface};
pub use self::code_section::*;
pub use self::custom_section::*;
pub use self::data_section::*;
pub use self::element_section::*;
pub use self::export_section::*;
pub use self::function_section::*;
pub use self::global_section::*;
pub use self::import_section::*;
pub use self::magic_and_version::*;
pub use self::memory_section::*;
pub use self::start_section::*;
pub use self::table_section::*;
pub use self::type_section::*;
