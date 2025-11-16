#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use libc::{c_char, c_float, c_int, c_uchar};
use std::{
    ffi::{CStr, CString},
    ptr,
};

mod ffi {
    unsafe extern "C" {
        fn make_demo(appname: &str) -> u32;
    }
}

/// The Scene Graph.
#[repr(C)]
pub struct SceneGraph {
    id: c_int,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
