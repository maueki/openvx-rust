#![feature(iterator_step_by)] 

extern crate cv;
extern crate vx;
extern crate vx_cv;

use vx::*;
use vx_cv::*;

use cv::*;
use cv::imgproc::*;
use cv::imgcodecs::*;
use cv::highgui::*;

fn main() {
    let img_path = "/home/maueki/Desktop/GitLab_Logo.svg.png";

    let im = Mat::from_path(img_path, ImageReadMode::Color).expect("Failed to read from path");

    if !im.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    let gray = im.cvt_color(ColorConversion::RGB2GRAY);

    let mut context = Context::new().unwrap();

    let input_image = create_image_from_mat(&context, &gray).unwrap();

    println!("create input image");

    let s = gray.size();
    let output_image = context.create_image(s.width as u32, s.height as u32, VX_DF_IMAGE_U8).unwrap();

    println!("create output image");

    // {
    //     let mut g = context.create_graph().unwrap();
    //     let graph_input = g.create_input(input_image).unwrap();
    //     let gau = g.gaussian_3x3(&graph_input).unwrap();

    //     g.set_output(&gau, &output_image).unwrap();
    //     g.verify().expect("verify failed");
    //     g.run().unwrap();
    // }

//    let out_mat = create_mat_from_image(&context, &output_image).unwrap();
    let out_mat = create_mat_from_image(&context, &input_image).unwrap();

    println!("end g.run()");

    highgui_named_window("Display Window", WindowFlag::Normal).unwrap();
    out_mat.show("Display Window", 0).unwrap();
}
