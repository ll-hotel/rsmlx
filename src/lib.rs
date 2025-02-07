use std::ffi::c_void;

pub mod core;
mod display;
mod image;
mod window;

pub struct Display {
    raw: *mut c_void,
}

pub use image::Image;
pub use window::{Hook, Window};

pub use x11::keysym;
pub use x11::xlib::{
    ButtonPress, ButtonPressMask, ButtonRelease, ButtonReleaseMask, DestroyAll, DestroyNotify,
    Expose, ExposureMask, KeyPress, KeyPressMask, KeyRelease, KeyReleaseMask, MotionNotify,
    PointerMotionMask,
};
