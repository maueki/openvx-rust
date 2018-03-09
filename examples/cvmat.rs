#![feature(iterator_step_by)] 

extern crate cv;
extern crate vx;

use vx::*;

use cv::*;
use cv::imgproc::*;
use cv::imgcodecs::*;
use cv::highgui::*;

use std::mem;
use std::os::raw::c_void;
use std::ptr::null_mut;
use std::os::raw::c_char;

fn create_mat_from_image(context: &Context, image: &Image) -> Result<Mat> {

    let mut width: u32 = 0;
    let mut height: u32 = 0;
    unsafe {
        vxQueryImage(image.ptr, vx_image_attribute_e::VX_IMAGE_WIDTH as i32,
                     &mut width as *mut _ as *mut c_void, mem::size_of_val(&width));
        vxQueryImage(image.ptr, vx_image_attribute_e::VX_IMAGE_HEIGHT as i32,
                     &mut height as *mut _ as *mut c_void, mem::size_of_val(&height));
    }
    println!("width, height = {}, {}", width, height);

    let rect = vx_rectangle_t{start_x:0, start_y:0, end_x:width, end_y:height};
    let out_mat = Mat::with_size(height as i32, width as i32, CvType::Cv8UC1 as i32);

    let ret_mat = unsafe {
        let mut buffer = vec![0u8; (width * height) as usize];

//        let step = out_mat.elem_size1() * out_mat.step1(0);
        let step = width;
        println!("step: {}", step);
        let mut src: *mut c_void = null_mut();
        let mut addr: vx_imagepatch_addressing_t = mem::uninitialized();
        let mut map_id: usize = 0;

        let map_res = vxMapImagePatch(image.ptr, &rect, 0, &mut map_id as *mut usize,
                                      &mut addr as *mut vx_imagepatch_addressing_t,
                                      &mut src,
                                      vx_accessor_e::VX_READ_ONLY as i32,
                                      vx_memory_type_e::VX_MEMORY_TYPE_HOST as i32,
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
        std::mem::forget(buffer);
        mat
    };

    Ok(ret_mat)
}

fn main() {
    let img_path = "/home/maueki/Desktop/GitLab_Logo.svg.png";

    let im = Mat::from_path(img_path, ImageReadMode::Color).expect("Failed to read from path");

    if !im.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    let gray = im.cvt_color(ColorConversion::RGB2GRAY);
    let s = gray.size();
    println!("width:{}, height:{}", s.width, s.height);

    let mut context = Context::new().unwrap();

    let in_data = gray.data() as *const _ as *const c_void;
    let input_image = unsafe {
        let mut addr: vx_imagepatch_addressing_t = mem::uninitialized();
        addr.dim_x = s.width as u32;
        addr.dim_y = s.height as u32;
//        addr.stride_x = gray.elem_size() as i32;
//        addr.stride_y = (gray.elem_size1() * gray.step1(0)) as i32;
        addr.stride_x = 1;
        addr.stride_y = s.width as i32;

        println!("in_data addr: {:?}", addr);
//        println!("in_data size: {}", in_data.len());
        let ptr = vxCreateImageFromHandle(context.ptr, VX_DF_IMAGE_U8,
                                          &addr, &in_data,
                                          vx_memory_type_e::VX_MEMORY_TYPE_HOST as i32);
        let res = vxGetStatus(ptr as vx_reference);
        if res != vx_status_e::VX_SUCCESS {
            println!("failed to vxCreateImageFromHandle: {}", res);
        }
        Image{ptr}
    };

    println!("create input image");

    let output_image = context.create_image(s.width as u32, s.height as u32, VX_DF_IMAGE_U8).unwrap();
    // let mut out_mat = Mat::with_size(s.height, s.width, CvType::Cv8UC1 as i32);
    // let out_data = out_mat.data();
    // let output_image = unsafe {
    //     let out_size = out_mat.size();
    //     let mut addr: vx_imagepatch_addressing_t = mem::uninitialized();
    //     addr.dim_x = out_size.width as u32;
    //     addr.dim_y = out_size.height as u32;
    //     addr.stride_x = out_mat.elem_size() as i32;
    //     addr.stride_y = (out_mat.elem_size1() * out_mat.step1(0)) as i32;

    //     println!("out_data addr: {:?}", addr);
    //     let ptr = vxCreateImageFromHandle(context.ptr, VX_DF_IMAGE_U8,
    //                                       &addr, out_data as *const _ as *const *const c_void, vx_memory_type_e::VX_MEMORY_TYPE_HOST as i32);
    //     let res = vxGetStatus(ptr as vx_reference);
    //     if res != vx_status_e::VX_SUCCESS {
    //         println!("failed to vxCreateImageFromHandle: {}", res);
    //     }
    //     Image{ptr}
    // };

    println!("create output image");

    {
        let mut g = context.create_graph().unwrap();
        let graph_input = g.create_input(input_image).unwrap();
        let gau = g.gaussian_3x3(&graph_input).unwrap();

        g.set_output(&gau, &output_image).unwrap();
        g.verify().expect("verify failed");
        g.run().unwrap();
    }

    let out_mat = create_mat_from_image(&context, &output_image).unwrap();

    println!("end g.run()");

    highgui_named_window("Display Window", WindowFlag::Normal).unwrap();
    out_mat.show("Display Window", 0).unwrap();
}
