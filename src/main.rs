
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate error_chain;

mod openvx;

use openvx::{Context, Graph};
use openvx::*;

fn edge_graph_factory(c: &mut Context) -> Result<Graph> {
    let kernels = vec![
        c.get_kernel_by_enum(VX_KERNEL_GAUSSIAN_3x3)?,
        c.get_kernel_by_enum(VX_KERNEL_SOBEL_3x3)?,
        c.get_kernel_by_enum(VX_KERNEL_MAGNITUDE)?,
    ];

    let g = c.create_graph()?;

    let virts = vec![
        g.create_virtual_image(0, 0, VX_DF_IMAGE_VIRT)?,
        g.create_virtual_image(0, 0, VX_DF_IMAGE_VIRT)?,
        g.create_virtual_image(0, 0, VX_DF_IMAGE_VIRT)?,
    ];

    let nodes = vec![
        g.create_generic_node(&kernels[0])?,
        g.create_generic_node(&kernels[1])?,
        g.create_generic_node(&kernels[2])?,
    ];

    let params = vec![
        nodes[0].get_parameter_by_index(0)?,
        nodes[2].get_parameter_by_index(2)?,
    ];

    for p in params.iter() {
        g.add_parameter(&*p).unwrap();
    }

    nodes[0].set_parameter_by_index(1, &virts[0]).unwrap();
    nodes[1].set_parameter_by_index(0, &virts[0]).unwrap();
    nodes[1].set_parameter_by_index(1, &virts[1]).unwrap();
    nodes[1].set_parameter_by_index(2, &virts[2]).unwrap();
    nodes[2].set_parameter_by_index(0, &virts[1]).unwrap();
    nodes[2].set_parameter_by_index(1, &virts[2]).unwrap();

    Ok(g)
}

fn main() {
    let mut context = Context::new().unwrap();

    let _graph = edge_graph_factory(&mut context).unwrap();
}
