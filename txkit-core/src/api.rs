/// Global C api state
use std::ffi::CString;
use std::sync::Mutex;

use lazy_static::lazy_static;

#[derive(Default)]
struct State {
    last_error: CString,
}

lazy_static! {
    static ref STATE: Mutex<State> = Mutex::new(State::new());
}

impl State {
    pub fn new() -> Self {
        env_logger::init_from_env(
            env_logger::Env::new()
                .filter_or("TXKIT_LOG", "opengl=debug,txkit=debug,tinygl=debug")
                .write_style("TXKIT_LOG"),
        );

        Self::default()
    }

    pub fn clear_last_error(&mut self) {
        self.last_error = CString::new("").unwrap();
    }

    pub fn set_last_error(&mut self, e: impl std::fmt::Display) {
        self.last_error = CString::new(e.to_string())
            .unwrap_or_else(|_| CString::new("invalid error message").unwrap());
    }
}

pub fn clear_last_error() {
    STATE.lock().unwrap().clear_last_error()
}

pub fn set_last_error(e: impl std::fmt::Display) {
    STATE.lock().unwrap().set_last_error(e);
}

pub fn wrap<T>(r: impl FnOnce() -> T) -> Option<T> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| r())) {
        Ok(result) => Some(result),
        Err(error) => {
            if let Some(message) = error.downcast_ref::<String>() {
                set_last_error(message);
            } else {
                // For debugging, don't just ignore the error if we can't print it
                panic!(error);
            }

            None
        }
    }
}

pub fn wrap_result<T, E: std::fmt::Display>(r: impl FnOnce() -> Result<T, E>) -> Option<T> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        r().map_err(|e| set_last_error(e))
            .map(|v| {
                clear_last_error();
                v
            })
            .ok()
    })) {
        Ok(result) => result,
        Err(error) => {
            if let Some(message) = error.downcast_ref::<String>() {
                set_last_error(message);
            } else {
                // For debugging, don't just ignore the error if we can't print it
                panic!(error);
            }

            None
        }
    }
}

pub fn wrap_result_code<E: std::fmt::Display>(r: impl FnOnce() -> Result<(), E>) -> i32 {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        r().map_err(|e| set_last_error(e))
            .map(|()| {
                clear_last_error();
                0
            })
            .unwrap_or(1)
    })) {
        Ok(result) => result,
        Err(error) => {
            if let Some(message) = error.downcast_ref::<String>() {
                set_last_error(message);
            } else {
                // For debugging, don't just ignore the error if we can't print it
                panic!(error);
            }

            1
        }
    }
}

/// No error occurred
pub const SUCCESS: i32 = 0;

/// Get the description of the last error that occurred in the txkit API
///
/// # Returns
///
/// Null pointer if no error occurred, or error message for the last error.
#[no_mangle]
pub extern "C" fn txkit_get_last_error() -> *const libc::c_char {
    let le = STATE.lock().unwrap().last_error.as_ptr();
    if unsafe { *le } == 0 {
        std::ptr::null()
    } else {
        le
    }
}
