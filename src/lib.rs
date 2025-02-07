use std::ffi::c_void;

pub mod core;
mod display;
mod window;
mod image;

pub struct Display {
    raw: *mut c_void,
}
