use std::any::Any;

use crate::context::Context;
use crate::error::*;
use crate::image::Image;

/// Represents a procedural texturing method
pub trait Method {
    fn compute(
        &mut self,
        ctx: &mut Context,
        tgt: &mut Image,
        params: Option<&dyn Any>,
    ) -> Result<()>;
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
pub extern "C" fn txkit_method_new(method_name: *const libc::c_char) -> *mut MethodBox {
    crate::api::wrap_result(|| {
        if method_name == std::ptr::null() {
            Err(Error::InvalidMethodName)
        } else {
            match unsafe { std::ffi::CStr::from_ptr(method_name as *const _) }.to_str() {
                #[cfg(feature = "method-debug")]
                Ok("debug") => Ok(Box::into_raw(Box::new(MethodBox {
                    method: Box::new(crate::methods::Debug::new()),
                }))),
                #[cfg(feature = "method-white-noise")]
                Ok("white_noise") => Ok(Box::into_raw(Box::new(MethodBox {
                    method: Box::new(crate::methods::WhiteNoise::new()),
                }))),
                Ok(_) => Err(Error::MethodNotFound),
                Err(_) => Err(Error::InvalidMethodName),
            }
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
/// * `params`: pointer to the parameter structure for this method
/// * `params_size`: size of the parameter structure
///
/// # Returns
///
/// TxKit_SUCCESS if no error occurred, else a non-zero code.
#[no_mangle]
pub unsafe extern "C" fn txkit_method_compute(
    ctx: &mut Context,
    method: &mut MethodBox,
    tgt: &mut Image,
    params: *const std::ffi::c_void,
    params_size: usize,
) -> i32 {
    let params_slice;
    let params: Option<&dyn Any> = if params == std::ptr::null() {
        None
    } else {
        params_slice = std::slice::from_raw_parts(params as *const u8, params_size);
        Some(&params_slice)
    };

    crate::api::wrap_result_code(|| method.method.compute(ctx, tgt, params))
}

/// Destroy a method
#[no_mangle]
pub unsafe extern "C" fn txkit_method_destroy(method: *mut MethodBox) {
    std::mem::drop(Box::from_raw(method))
}