#![allow(dead_code)]

use std::fmt::Display;

#[cfg(test)]
macro_rules! assert_snapshot {
    ($body:expr) => {
        if cfg!(feature = "__assert_snapshots") {
            insta::assert_snapshot!($body);
        } else {
            let _ = $body;
        }
    };
}

#[cfg(test)]
macro_rules! assert_debug_snapshot {
    ($body:expr) => {
        if cfg!(feature = "__assert_snapshots") {
            insta::assert_debug_snapshot!($body);
        } else {
            let _ = $body;
        }
    };
}
