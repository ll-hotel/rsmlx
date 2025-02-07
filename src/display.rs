use std::{ffi::c_void, rc::Rc};

use crate::{core::{free, mlx_destroy_display, mlx_init, mlx_loop, mlx_loop_end, mlx_loop_hook}, Display};

impl Display {
    pub fn new() -> Option<Rc<Self>> {
        let mlx_ptr = unsafe { mlx_init() };
        if mlx_ptr.is_null() {
            return None;
        }
        Some(Rc::new(Self { raw: mlx_ptr }))
    }
    pub fn loop_hook(&self, callback: extern "C" fn(*mut c_void), callback_param: *mut c_void) {
        unsafe {
            mlx_loop_hook(self.raw, Some(callback), callback_param);
        }
    }
    pub fn r#loop(&self) {
        unsafe {
            mlx_loop(self.raw);
        }
    }
    pub fn loop_end(&self) {
        unsafe {
            mlx_loop_end(self.raw);
        }
    }
}
impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            mlx_destroy_display(self.raw);
            free(self.raw);
        }
    }
}
