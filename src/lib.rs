use std::ffi::c_void;
use std::rc::Rc;

pub mod core;
mod display;
mod image;
mod window;

pub struct Display {
    raw: *mut c_void,
}

pub struct Window {
    display: Rc<Display>,
    width: u32,
    height: u32,
    raw: *mut c_void,
}

pub struct Image {
    display: Rc<Display>,
    width: u32,
    height: u32,
    bpp: u32,
    size_line: u32,
    endian: u32,
    addr: *mut i8,
    raw: *mut c_void,
}

pub use x11::keysym;
pub use x11::xlib::{
    ButtonPress, ButtonPressMask, ButtonRelease, ButtonReleaseMask, DestroyAll, DestroyNotify,
    Expose, ExposureMask, KeyPress, KeyPressMask, KeyRelease, KeyReleaseMask, MotionNotify,
    PointerMotionMask,
};
