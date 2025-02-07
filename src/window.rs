#![allow(non_upper_case_globals)]

use std::{ffi::c_void, rc::Rc};

use crate::{
    core::{
        mlx_clear_window, mlx_destroy_window, mlx_expose_hook, mlx_hook, mlx_key_hook,
        mlx_mouse_hide, mlx_mouse_hook, mlx_mouse_show, mlx_new_window, mlx_put_image_to_window,
    },
    ButtonPress, ButtonRelease, Display, Image, KeyPress, KeyRelease, MotionNotify, Window,
};

impl Window {
    pub fn new(display: Rc<Display>, width: u32, height: u32, title: &str) -> Option<Self> {
        let win_ptr = unsafe {
            mlx_new_window(
                display.raw,
                width as i32,
                height as i32,
                title.as_ptr() as *mut i8,
            )
            .as_mut()?
        };
        Some(Self {
            display,
            width,
            height,
            raw: win_ptr,
        })
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn key_hook(&self, callback: extern "C" fn(i32, *mut c_void), callback_param: *mut c_void) {
        unsafe { mlx_key_hook(self.raw, Some(callback), callback_param) };
    }
    pub fn mouse_hook(
        &self,
        callback: extern "C" fn(i32, i32, i32, *mut c_void),
        callback_param: *mut c_void,
    ) {
        unsafe { mlx_mouse_hook(self.raw, Some(callback), callback_param) };
    }
    pub fn expose_hook(&self, callback: extern "C" fn(*mut c_void), callback_param: *mut c_void) {
        unsafe { mlx_expose_hook(self.raw, Some(callback), callback_param) };
    }
    pub fn hook(&self, x_event: i32, x_mask: i32, callback: Hook, callback_param: *mut c_void) -> Result<(), String> {
        let funct: extern "C" fn();
        match x_event {
            KeyPress | KeyRelease => {
                let Hook::Key(_hook) = callback else {
                    return Err("invalid window hook: expected Hook::Key".to_owned());
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
            ButtonPress | ButtonRelease => {
                let Hook::MouseButton(_hook) = callback else {
                    return Err("invalid window hook: expected Hook::MouseButton".to_owned());
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
            MotionNotify => {
                let Hook::MouseMotion(_hook) = callback else {
                    return Err("invalid window hook: expected Hook::MouseMotion".to_owned());
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
            // Generic
            _ => {
                let Hook::Generic(_hook) = callback else {
                    return Err("invalid window hook: expected Hook::Generic".to_owned());
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
        }
        unsafe { mlx_hook(self.raw, x_event, x_mask, Some(funct), callback_param) };
        Ok(())
    }
    pub fn put_image(&self, image: &Image, x: i32, y: i32) {
        unsafe { mlx_put_image_to_window(self.display.raw, self.raw, image.raw, x, y) };
    }
    pub fn clear(&self) {
        unsafe { mlx_clear_window(self.display.raw, self.raw) };
    }
    pub fn mlx_mouse_hide(&self) {
        unsafe { mlx_mouse_hide(self.display.raw, self.raw) };
    }
    pub fn mlx_mouse_show(&self) {
        unsafe { mlx_mouse_show(self.display.raw, self.raw) };
    }
}
impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            mlx_destroy_window(self.display.raw, self.raw);
        }
    }
}

pub enum Hook {
    Key(extern "C" fn(i32, *mut c_void)),
    MouseButton(extern "C" fn(i32, i32, i32, *mut c_void)),
    MouseMotion(extern "C" fn(*mut c_void)),
    Generic(extern "C" fn(*mut c_void)),
}
