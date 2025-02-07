use std::{ffi::c_void, rc::Rc};

use crate::{
    core::{
        mlx_destroy_window, mlx_expose_hook, mlx_hook, mlx_key_hook, mlx_mouse_hook, mlx_new_window,
    },
    Display,
};

pub struct Window {
    display: Rc<Display>,
    width: u32,
    height: u32,
    raw: *mut c_void,
}
impl Window {
    pub fn new(display: Rc<Display>, width: u32, height: u32, title: &str) -> Option<Self> {
        let win_ptr = unsafe {
            mlx_new_window(
                display.raw,
                width as i32,
                height as i32,
                title.as_ptr() as *mut i8,
            )
        };
        if win_ptr.is_null() {
            return None;
        }
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
        unsafe {
            mlx_key_hook(self.raw, Some(callback), callback_param);
        }
    }
    pub fn mouse_hook(
        &self,
        callback: extern "C" fn(i32, i32, i32, *mut c_void),
        callback_param: *mut c_void,
    ) {
        unsafe {
            mlx_mouse_hook(self.raw, Some(callback), callback_param);
        }
    }
    pub fn expose_hook(&self, callback: extern "C" fn(*mut c_void), callback_param: *mut c_void) {
        unsafe {
            mlx_expose_hook(self.raw, Some(callback), callback_param);
        }
    }
    pub fn hook(
        &self,
        x_event: i32,
        x_mask: i32,
        callback: WindowHook,
        callback_param: *mut c_void,
    ) {
        let funct: extern "C" fn();
        match x_event {
            // X11 Key events
            2 | 3 => {
                let WindowHook::Key(_hook) = callback else {
                    panic!("invalid window hook: expected WindowHook::Key");
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
            // X11 Mouse button events
            4 | 5 => {
                let WindowHook::MouseButton(_hook) = callback else {
                    panic!("invalid window hook: expected WindowHook::MouseButton");
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
            6 => {
                let WindowHook::MouseMotion(_hook) = callback else {
                    panic!("invalid window hook: expected WindowHook::MouseMotion");
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
            // Generic
            _ => {
                let WindowHook::Generic(_hook) = callback else {
                    panic!("invalid window hook: expected WindowHook::Generic");
                };
                funct = unsafe { std::mem::transmute(_hook) };
            }
        }
        unsafe {
            mlx_hook(self.raw, x_event, x_mask, Some(funct), callback_param);
        }
    }
}
impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            mlx_destroy_window(self.display.raw, self.raw);
        }
    }
}

pub enum WindowHook {
    Key(extern "C" fn(i32, *mut c_void)),                   // 2, 3
    MouseButton(extern "C" fn(i32, i32, i32, *mut c_void)), // 4, 5
    MouseMotion(extern "C" fn(*mut c_void)),                // 6
    Generic(extern "C" fn(*mut c_void)),
}
