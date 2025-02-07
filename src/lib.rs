pub mod core;

use std::{ffi::c_void, rc::Rc};

use core::{
    free, mlx_destroy_display, mlx_destroy_image, mlx_destroy_window, mlx_get_data_addr, mlx_init,
    mlx_new_image, mlx_new_window,
};

pub struct Display {
    raw: *mut c_void,
}
impl Display {
    pub fn new() -> Option<Rc<Self>> {
        let mlx_ptr = unsafe { mlx_init() };
        if mlx_ptr.is_null() {
            return None;
        }
        Some(Rc::new(Self { raw: mlx_ptr }))
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
}
impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            mlx_destroy_window(self.display.raw, self.raw);
        }
    }
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
impl Image {
    pub fn new(display: Rc<Display>, width: u32, height: u32) -> Option<Self> {
        let img_ptr = unsafe { mlx_new_image(display.raw, width as i32, height as i32) };
        if img_ptr.is_null() {
            return None;
        }
        let (mut bpp, mut size_line, mut endian) = (0, 0, 0);
        let addr = unsafe { mlx_get_data_addr(img_ptr, &mut bpp, &mut size_line, &mut endian) };
        if addr.is_null() {
            unsafe { mlx_destroy_image(display.raw, img_ptr) };
            return None;
        }
        Some(Self {
            display,
            width,
            height,
            bpp: bpp as u32,
            size_line: size_line as u32,
            endian: endian as u32,
            addr,
            raw: img_ptr,
        })
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn bpp(&self) -> u32 {
        self.bpp
    }
    pub fn size_line(&self) -> u32 {
        self.size_line
    }
    pub fn endian(&self) -> u32 {
        self.endian
    }
    pub fn addr(&self) -> *mut i8 {
        self.addr
    }
}
impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            mlx_destroy_image(self.display.raw, self.raw);
        }
    }
}
