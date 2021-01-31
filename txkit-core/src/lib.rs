//! [![Build](https://github.com/vtavernier/txkit/workflows/Build/badge.svg?branch=master)](https://github.com/vtavernier/txkit/actions)
//!
//! TextureKit (txkit) is an implementation of common procedural texturing techniques used in
//! computer graphics. It's a Rust library which can be used from other Rust programs as well as
//! through its C API.

#[macro_use]
pub mod context;
mod error;
pub mod image;
pub mod io;
pub mod method;

pub use error::{Error, Result};
