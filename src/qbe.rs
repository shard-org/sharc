//! qbe wrapper.

// NOTE: this implementation is a mess, we'll keep it to experiment
// with qbe codegen until a better alternative shows up

use enum_primitive::*;
use struct_primitive::*;

enum_from_primitive! {
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Target {
    T_amd64_sysv,
    T_amd64_apple,
    T_arm64,
    T_arm64_apple,
    T_rv64,
}
}

extern "C" {
    /// main qbe command
    pub fn qbe();
}
