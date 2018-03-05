
extern crate openvx;

use openvx::Context;
use openvx::*;

fn main() {
    let mut context = Context::new().unwrap();

    let input_image = context.create_image(640, 480, VX_DF_IMAGE_U8).unwrap();
    let output_image = context.create_image(640, 480, VX_DF_IMAGE_S16).unwrap();

    let mut g = context.create_graph().unwrap();

    let graph_input = g.create_input(input_image).unwrap();

    let gau = g.gaussian_3x3(&graph_input).unwrap();
    let (sobel_x, sobel_y) = g.sobel_3x3(&gau).unwrap();
    let mag = g.magnitude(&sobel_x, &sobel_y).unwrap();

    g.set_output(&mag, &output_image).unwrap();

    g.verify().unwrap();

    g.run().unwrap();
}
