
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
pub union vx_pixel_value_t {
    pub RGB: [u8; 3usize],
    pub RGBX: [u8; 4usize],
    pub YUV: [u8; 3usize],
    pub U8: u8,
    pub U16: u16,
    pub S16: i16,
    pub U32: u32,
    pub S32: i32,
    pub reserved: [u8; 16usize],
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    let _context = unsafe {vxCreateContext()};

}
