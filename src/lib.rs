#[macro_use]
mod util;

pub mod error;
pub mod intern;
pub mod span;

// TODO: Make this private
pub mod internal {
    pub mod state;
    pub mod syntax;
}

pub use beef::lean::Cow;
