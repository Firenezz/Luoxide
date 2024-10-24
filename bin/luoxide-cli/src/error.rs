use std::path::PathBuf;


#[derive(Debug, Eq, PartialEq, Error, Clone)]
pub enum Error {
    #[error("failed to parse Lua source code")]
    Parse {
        path: PathBuf,
        src: EcoString,
        error: luoxide_parser::error::ParseError
    }
}