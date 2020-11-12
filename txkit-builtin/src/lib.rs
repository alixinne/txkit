pub mod methods;

#[cfg(feature = "wrap-shaders")]
#[allow(dead_code)]
mod shaders {
    include!(concat!(env!("OUT_DIR"), "/shaders.rs"));
}
