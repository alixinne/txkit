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

    pub fn set_last_error(&mut self, e: impl ToString) {
        self.last_error = CString::new(e.to_string())
            .unwrap_or_else(|_| CString::new("invalid error message").unwrap());
    }
}

pub fn clear_last_error() {
    STATE.lock().unwrap().clear_last_error()
}

pub fn set_last_error(e: impl ToString) {
    STATE.lock().unwrap().set_last_error(e);
}

pub fn wrap_result<T, E: ToString>(r: Result<T, E>) -> Option<T> {
    r.map_err(|e| set_last_error(e))
        .map(|v| {
            clear_last_error();
            v
        })
        .ok()
}

pub fn wrap_result_code<E: ToString>(r: Result<(), E>) -> i32 {
    r.map_err(|e| set_last_error(e))
        .map(|()| {
            clear_last_error();
            0
        })
        .unwrap_or(1)
}

/// No error occurred
pub const TXKIT_SUCCESS: i32 = 0;

/// Get the description of the last error that occurred in the txkit API
///
/// # Returns
///
/// Null pointer if no error occurred, or error message for the last error.
#[no_mangle]
pub extern "C" fn txkit_get_last_error() -> *const i8 {
    let le = STATE.lock().unwrap().last_error.as_ptr();
    if unsafe { *le } == 0 {
        std::ptr::null()
    } else {
        le
    }
}
