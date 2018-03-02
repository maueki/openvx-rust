
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate error_chain;

mod openvx;

use openvx::{Context, Graph};
use openvx::*;

fn vx_edge_graph_factory(c: &mut Context) -> Result<Graph> {
    let _kernels = [
        c.get_kernel_by_enum(vx_kernel_e_VX_KERNEL_GAUSSIAN_3x3 as i32),
        c.get_kernel_by_enum(vx_kernel_e_VX_KERNEL_SOBEL_3x3 as i32),
        c.get_kernel_by_enum(vx_kernel_e_VX_KERNEL_MAGNITUDE as i32),
    ];

    let g = c.create_graph()?;

    let _virts = [
        g.create_virtual_image(0, 0, vx_df_image_e_VX_DF_IMAGE_VIRT),
        g.create_virtual_image(0, 0, vx_df_image_e_VX_DF_IMAGE_VIRT),
        g.create_virtual_image(0, 0, vx_df_image_e_VX_DF_IMAGE_VIRT),
    ];

//         let nodes = [
//             vxCreateGenericNode(g, kernels[0]),
//             vxCreateGenericNode(g, kernels[1]),
//             vxCreateGenericNode(g, kernels[2]),
//         ];

//         let params = [
//             vxGetParameterByIndex(nodes[0], 0),
//             vxGetParameterByIndex(nodes[2], 2),
//         ];

//         let mut status = vx_status_e_VX_SUCCESS;
//         for p in params.iter() {
//             status |= vxAddParameterToGraph(g, *p);
//         }

//         status |= vxSetParameterByIndex(nodes[0], 1, virts[0] as vx_reference);
//         status |= vxSetParameterByIndex(nodes[1], 0, virts[0] as vx_reference);
//         status |= vxSetParameterByIndex(nodes[1], 1, virts[1] as vx_reference);
//         status |= vxSetParameterByIndex(nodes[1], 2, virts[2] as vx_reference);
//         status |= vxSetParameterByIndex(nodes[2], 0, virts[1] as vx_reference);
//         status |= vxSetParameterByIndex(nodes[2], 1, virts[2] as vx_reference);

//         for v in virts.iter() {
// //            vxReleaseImage(*v);
//         }

//         if status != vx_status_e_VX_SUCCESS {
//             println!("Failed to create graph in factory!");
// //            vxReleaseGraph(&mut g);
//         }

    Ok(g)
}

fn main() {
    let context = Context::new();
}
