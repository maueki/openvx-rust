#![feature(iterator_step_by)]

extern crate cv;
extern crate vx;
extern crate vx_api;

use cv::*;
use vx::*;
use vx_api::*;

use vx_memory_type_e::*;
use vx_accessor_e::*;

use std::mem;
use std::ptr::null_mut;
use std::os::raw::c_void;

// TODO: Support multi planar image
pub fn create_mat_from_image(context: &Context, image: &Image) -> Result<Mat> {

    let mut width: u32 = image.query::<u32>(VX_IMAGE_WIDTH)?;
    let mut height: u32 = image.query::<u32>(VX_IMAGE_HEIGHT)?;

    println!("width, height = {}, {}", width, height);

    let rect = vx_rectangle_t{start_x:0, start_y:0, end_x:width, end_y:height};

    unsafe {
        let mut buffer = vec![0u8; (width * height) as usize];

        let step = width;
        println!("step: {}", step);
        let mut src: *mut c_void = null_mut();
        let mut addr: vx_imagepatch_addressing_t = mem::uninitialized();
        let mut map_id: usize = 0;

        let map_res = vxMapImagePatch(image.ptr, &rect, 0, &mut map_id as *mut usize,
                                      &mut addr as *mut vx_imagepatch_addressing_t,
                                      &mut src,
                                      VX_READ_ONLY as i32,
                                      VX_MEMORY_TYPE_HOST as i32,
                                      0);
        if map_res != vx_status_e::VX_SUCCESS {
            println!("@@@ map failed");
        }

        println!("addr: {:?}", addr);
        let len = addr.stride_x * (addr.dim_x * addr.scale_x) as i32 / VX_SCALE_UNITY as i32;

        println!("len: {}", len);

        {
            let p = &mut buffer[0] as *mut u8;
            for y in (0..height).step_by(addr.step_y as usize) {
                let ptr = vxFormatImagePatchAddress2d(src as *mut c_void, 0, y - rect.start_y, &addr);
                std::ptr::copy_nonoverlapping(ptr,
                                              p.offset((y * step as u32) as isize) as *mut c_void,
                                              len as usize);
            }
        }
        vxUnmapImagePatch(image.ptr, map_id);

        let mat = Mat::from_buffer(height as i32, width as i32, CvType::Cv8UC1 as i32, &buffer);

        // !!! Mat::from_buffer doesn't take ownership, but Mat::drop disposes buffer.
        std::mem::forget(buffer);
        Ok(mat)
    }
}

// TODO: Support multi planar image
pub fn create_image_from_mat(context: &Context, mat: &Mat) -> Result<Image> {
    unsafe {
        let s = mat.size();
        let mut addr: vx_imagepatch_addressing_t = mem::uninitialized();
        addr.dim_x = s.width as u32;
        addr.dim_y = s.height as u32;
        addr.stride_x = mat.elem_size() as i32;
        addr.stride_y = (mat.elem_size1() * mat.step1(0)) as i32;

        println!("create_iamge_from_mat stride_x:{}, stride_y:{}", addr.stride_x, addr.stride_y);

        context.create_image_from_buffer(VX_DF_IMAGE_U8,
                                         &addr, mat.data())

        // let in_data = mat.data() as *const _ as *const c_void;
        // let ptr = vxCreateImageFromHandle(context.ptr, VX_DF_IMAGE_U8,
        //                                   &addr, &in_data,
        //                                   VX_MEMORY_TYPE_HOST as i32);
        // let res = vxGetStatus(ptr as vx_reference);
        // if res != vx_status_e::VX_SUCCESS {
        //     println!("failed to vxCreateImageFromHandle: {}", res);
        // }
        // Ok(Image{ptr})
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
