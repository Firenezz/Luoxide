use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to parse Lua source code")]
    Parse {
        path: PathBuf,
        //src: EcoString,
        error: luoxide_parser::error::ParseError,
    },
}
