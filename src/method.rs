use failure::Fail;

use crate::context::Context;
use crate::image::Image;

#[derive(Fail, Debug, Clone, Eq, PartialEq)]
pub enum Error {
    #[fail(display = "the method doesn't support the given context")]
    ContextNotSupported,
    #[fail(display = "the method doesn't support the requested format")]
    FormatNotSupported,
    #[fail(display = "the requested method was not found")]
    MethodNotFound,
    #[fail(display = "invalid method name")]
    InvalidMethodName,
    #[fail(display = "context creation failed: {}", 0)]
    ContextCreationFailed(String),
    #[fail(display = "method initialization failed: {}", 0)]
    MethodInitializationFailed(String),
}

/// Represents a procedural texturing method
pub trait Method {
    fn compute(&mut self, ctx: &mut Context, tgt: &mut Image) -> Result<(), Error>;
}

/// Wrapped method for FFI
pub struct MethodBox {
    method: Box<dyn Method>,
}

/// Create a new method by name
///
/// # Parameters
///
/// * `method_name`: name of the method to create
///
/// # Returns
///
/// Null pointer if an error occurred creating the method, otherwise pointer to the allocated
/// method.
#[no_mangle]
pub extern "C" fn txkit_method_new(method_name: *const u8) -> *mut MethodBox {
    crate::api::wrap_result(if method_name == std::ptr::null() {
        Err(Error::InvalidMethodName)
    } else {
        match unsafe { std::ffi::CStr::from_ptr(method_name as *const _) }.to_str() {
            Ok("debug") => Ok(Box::into_raw(Box::new(MethodBox {
                method: Box::new(crate::methods::Debug::new()),
            }))),
            Ok("whitenoise") => Ok(Box::into_raw(Box::new(MethodBox {
                method: Box::new(crate::methods::Whitenoise::new()),
            }))),
            Ok(_) => Err(Error::MethodNotFound),
            Err(_) => Err(Error::InvalidMethodName),
        }
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Compute an image using the given method
///
/// # Parameters
///
/// * `ctx`: context to use for computing the image
/// * `method`: texturing method
/// * `tgt`: target image to be computed
///
/// # Returns
///
/// TXKIT_SUCCESS if no error occurred, else a non-zero code.
#[no_mangle]
pub extern "C" fn txkit_method_compute(
    ctx: &mut Context,
    method: &mut MethodBox,
    tgt: &mut Image,
) -> i32 {
    crate::api::wrap_result_code(method.method.compute(ctx, tgt))
}

/// Destroy a method
#[no_mangle]
pub unsafe extern "C" fn txkit_method_destroy(method: *mut MethodBox) {
    std::mem::drop(Box::from_raw(method))
}
