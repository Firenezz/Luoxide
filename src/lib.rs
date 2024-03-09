#[macro_use]
mod util;

pub mod intern;
pub mod span;
pub mod error;

mod internal {
    pub(crate) mod syntax;
}
