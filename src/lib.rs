//! TextureKit (txkit) is an implementation of common procedural texturing techniques used in
//! computer graphics. It's a Rust library which can be used from other Rust programs as well as
//! through its C API.

pub mod api;
pub mod context;
pub mod image;
pub mod method;
pub mod methods;

#[cfg(feature = "gpu")]
#[allow(dead_code)]
mod shaders {
    include!(concat!(env!("OUT_DIR"), "/shaders.rs"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
