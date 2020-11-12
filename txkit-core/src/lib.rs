//! [![Build Status](https://travis-ci.com/vtavernier/txkit.svg?branch=master)](https://travis-ci.com/vtavernier/txkit)
//!
//! TextureKit (txkit) is an implementation of common procedural texturing techniques used in
//! computer graphics. It's a Rust library which can be used from other Rust programs as well as
//! through its C API.

pub mod api;
#[macro_use]
pub mod context;
mod error;
pub mod image;
pub mod method;

pub use error::{Error, Result};
