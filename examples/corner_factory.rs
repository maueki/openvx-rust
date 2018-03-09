extern crate vx;

use vx::*;

fn main() {

    let mut context = Context::new().unwrap();

    let input_image = context.create_image(640, 480, VX_DF_IMAGE_YUYV).unwrap();
    let output_array = context.create_array(VX_TYPE_KEYPOINT, 1000).unwrap();

    let mut g = context.create_graph().unwrap();

    let graph_input = g.create_input(input_image).unwrap();

    let channel = g.channel_extract(&graph_input, &VX_CHANNEL_Y).unwrap();
    let median = g.median_3x3(&channel).unwrap();
    let (corners, _) = g.harris_corners(
        &median,
        &10000.0f32, // strength_thresh
        &1.5f32, // min_distance
        &0.14f32, // sensitivity
        &3, // window_size
        &3, // block_size
    ).unwrap();

    g.set_output_array(&corners, &output_array).unwrap();

    g.verify().unwrap();

    g.run().unwrap();
}
