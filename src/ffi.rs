#![allow(dead_code)]

use std::os::raw::{c_int, c_uint};

unsafe extern "C" {
    pub fn plc_init() -> c_int;

    pub fn plc_read_u32(
        address: c_uint,
        value: *mut c_uint,
    ) -> c_int;

    pub fn plc_write_u32(
        address: c_uint,
        value: c_uint,
    ) -> c_int;

    pub fn plc_shutdown() -> c_int;
}
