pub use txkit_builtin as builtin;
pub use txkit_core as core;

pub mod config {
    include!(concat!(env!("OUT_DIR"), "/config.rs"));
}
