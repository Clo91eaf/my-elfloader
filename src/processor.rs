use std::ffi::c_void;
extern crate libloading;

#[repr(C)]
#[derive(Debug)]
pub struct Processor {}

#[link(name = "processor", kind = "static")]
extern "C" {
}
