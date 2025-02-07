use std::{ffi::c_void, rc::Rc};

use crate::{
    core::{mlx_destroy_image, mlx_get_data_addr, mlx_new_image},
    Display,
};

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
