//! Definition of the method registry type

use std::collections::HashMap;

use crate::Error;

use super::{Method, MethodBox};

/// Type of a method constructor
pub type MethodConstructor = Box<dyn Fn() -> Box<dyn Method>>;

/// Registry to declare methods by name
#[derive(Default)]
pub struct MethodRegistry {
    method_constructors: HashMap<String, MethodConstructor>,
}

impl MethodRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn into_box(self) -> RegistryBox {
        RegistryBox {
            registry: Box::new(self),
        }
    }

    pub fn register(&mut self, name: &str, constructor: MethodConstructor) {
        self.method_constructors
            .insert(name.to_string(), constructor);
    }

    pub fn build(&self, name: &str) -> Option<Box<dyn Method>> {
        self.method_constructors.get(name).and_then(|v| Some(v()))
    }
}

pub struct RegistryBox {
    registry: Box<MethodRegistry>,
}

/// Create a new method by name
///
/// # Parameters
///
/// * `registry`: registry of methods to build from
/// * `method_name`: name of the method to create
///
/// # Returns
///
/// Null pointer if an error occurred creating the method, otherwise pointer to the allocated
/// method.
#[no_mangle]
pub extern "C" fn txkit_method_new(
    registry: &RegistryBox,
    method_name: *const libc::c_char,
) -> *mut MethodBox {
    crate::api::wrap_result(|| {
        if method_name == std::ptr::null() {
            Err(Error::InvalidMethodName)
        } else {
            match unsafe { std::ffi::CStr::from_ptr(method_name as *const _) }.to_str() {
                Ok(method) => {
                    if let Some(method) = registry.registry.build(method) {
                        Ok(Box::into_raw(Box::new(MethodBox { method })))
                    } else {
                        Err(Error::MethodNotFound)
                    }
                }
                Err(_) => Err(Error::InvalidMethodName),
            }
        }
    })
    .unwrap_or(std::ptr::null_mut())
}

/// Destroy a registry
#[no_mangle]
pub unsafe extern "C" fn txkit_registry_destroy(registry: *mut RegistryBox) {
    std::mem::drop(Box::from_raw(registry))
}
