#[macro_use]
mod util;

pub mod error;
pub mod intern;
pub mod span;

mod internal {
    pub(crate) mod syntax;
}
