mod debug;
pub use debug::*;

mod white_noise;
pub use white_noise::*;

mod value_noise;
pub use value_noise::*;

mod gradient_noise;
pub use gradient_noise::*;

use txkit_core::method::{MethodRegistry, RegistryBox};
pub fn new_registry() -> MethodRegistry {
    let mut registry = MethodRegistry::new();
    registry.register("debug", Box::new(|| Box::new(Debug::new())));
    registry.register("white_noise", Box::new(|| Box::new(WhiteNoise::new())));
    registry.register("value_noise", Box::new(|| Box::new(ValueNoise::new())));
    registry.register(
        "gradient_noise",
        Box::new(|| Box::new(GradientNoise::new())),
    );
    registry
}

/// Create a new registry with txkit built-in methods registered
///
/// # Returns
///
/// Pointer to the allocated registry.
#[no_mangle]
pub extern "C" fn txkit_registry_new_builtin() -> *mut RegistryBox {
    txkit_core::api::wrap(|| Box::into_raw(Box::new(new_registry().into_box())))
        .unwrap_or(std::ptr::null_mut())
}
