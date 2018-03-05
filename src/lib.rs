#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[macro_use]
extern crate error_chain;

#[allow(non_camel_case_types,
        non_upper_case_globals,
        non_snake_case,
        dead_code)]
mod ffi {

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
}

use self::ffi::*;

pub use self::ffi::vx_kernel_e::*;
pub use self::ffi::vx_df_image_e::*;
pub use self::ffi::vx_graph_attribute_e::*;

use self::ffi::vx_status_e::*;

use std::mem;
use std::os::raw::{c_void};
use std::rc::Rc;

error_chain! {
    errors {
        ErrorNotImplemented {
        }
    }
}

pub trait Reference {
    fn to_ref(&self) -> vx_reference;
}

pub trait InputImage {
    fn set_input_image(&self, graph: &mut Graph, node: &Node, param_index: usize) -> Result<()>;
}

pub struct Context {
    ptr: vx_context
}

fn convert_error(_err: vx_status) -> ErrorKind {
    ErrorKind::ErrorNotImplemented
}

impl Context {
    pub fn new() -> Result<Self> {
        unsafe {
            let ptr = vxCreateContext();
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Context{ptr})
        }
    }

    pub fn create_graph(&mut self) -> Result<Graph> {
        unsafe {
            let ptr = vxCreateGraph(self.ptr);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Graph::new(ptr, self))
        }
    }

    pub fn get_kernel_by_enum(&self, kernel: vx_kernel_e::Type) -> Result<Kernel> {
        unsafe {
            let ptr = vxGetKernelByEnum(self.ptr, kernel as i32);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Kernel{ptr})
        }
    }

    pub fn create_image(&self, width: u32, height: u32, color: vx_df_image_e::Type) -> Result<Image> {
        unsafe {
            let ptr = vxCreateImage(self.ptr, width, height, color);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Image{ptr})
        }
    }

}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            let res = vxReleaseContext(&mut self.ptr);
            if res != VX_SUCCESS {
                // TODO: 
            }
        }
    }
}

pub struct Graph<'a> {
    ptr: vx_graph,
    ctx: &'a Context,
    param_num: usize,
}

impl<'a> Graph<'a> {

    fn new(ptr: vx_graph, ctx: &'a Context) -> Self {
        Graph{ptr: ptr, ctx: ctx, param_num: 0}
    }

    fn create_virtual_image(&self, width: u32, height: u32, color: vx_df_image_e::Type) -> Result<Image> {
        unsafe {
            let ptr = vxCreateVirtualImage(self.ptr, width, height, color);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Image{ptr})
        }
    }

    fn create_generic_node(&self, kernel: &Kernel) -> Result<Node> {
        unsafe {
            let ptr = vxCreateGenericNode(self.ptr, kernel.ptr);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Node{ptr: ptr})
        }
    }

    fn add_parameter(&mut self, param: &NodeParam) -> Result<usize> {
        unsafe {
            let res = vxAddParameterToGraph(self.ptr, param.ptr);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            let param_index = self.param_num;
            self.param_num += 1;

            Ok(param_index)
        }
    }

    fn query<T>(&self, attribute: vx_graph_attribute_e::Type) -> Result<T> {
        unsafe {
            let mut val: T = mem::uninitialized();
            let ptr: *mut c_void = &mut val as *mut _ as *mut c_void;
            let res = vxQueryGraph(self.ptr, attribute as i32, ptr, mem::size_of_val(&val));
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(val)
        }
    }

    fn set_parameter_by_index(&self, index: usize, reference: &Reference) -> Result<()> {
        unsafe {
            let res = vxSetGraphParameterByIndex(self.ptr, index as u32, reference.to_ref());
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(())
        }
    }

    pub fn gaussian_3x3(&mut self, input: &InputImage) -> Result<NodeOutputImage> {

        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_GAUSSIAN_3x3)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        input.set_input_image(self, &node, 0).unwrap();

        let out = NodeOutputImage{node: node.clone(), param_index: 1};

        Ok(out)
    }

    pub fn sobel_3x3(&mut self, input: &InputImage) -> Result<(NodeOutputImage, NodeOutputImage)> {

        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_SOBEL_3x3)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        input.set_input_image(self, &node, 0).unwrap();

        let out1 = NodeOutputImage{node: node.clone(), param_index: 1};
        let out2 = NodeOutputImage{node: node.clone(), param_index: 2};


        Ok((out1, out2))
    }

    pub fn magnitude(&mut self, grad_x: &InputImage, grad_y: &InputImage) -> Result<NodeOutputImage> {
        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_MAGNITUDE)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        grad_x.set_input_image(self, &node, 0).unwrap();
        grad_y.set_input_image(self, &node, 1).unwrap();

        let out = NodeOutputImage{node: node.clone(), param_index: 2};
        Ok(out)
    }

    pub fn create_input(&self, image: Image) -> Result<GraphInput> {
        Ok(GraphInput{image: image})
    }

    pub fn set_output(&mut self, output: &NodeOutputImage, image: &Image) -> Result<()> {
        let param = output.node.get_parameter_by_index(output.param_index as u32).unwrap();
        let param_index = self.add_parameter(&param).unwrap();

        self.set_parameter_by_index(param_index, image).unwrap();
        Ok(())
    }

    pub fn verify(&self) -> Result<()> {
        unsafe {
            let res = vxVerifyGraph(self.ptr);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(())
        }
    }

    pub fn run(&self) -> Result<()> {
        unsafe {
            let res = vxProcessGraph(self.ptr);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(())
        }
    }
}

impl<'a> Drop for Graph<'a> {
    fn drop(&mut self) {
        unsafe {
            let res = vxReleaseGraph(&mut self.ptr);
            if res != VX_SUCCESS {
                // TODO: 
            }
        }
    }
}

pub struct Image {
    ptr: vx_image
}

impl Image {
}

impl Reference for Image {
    fn to_ref(&self) -> vx_reference {
        self.ptr as vx_reference
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe {
            let res = vxReleaseImage(&mut self.ptr);
            if res != VX_SUCCESS {
                // TODO: 
            }
        }
    }
}

pub struct Kernel {
    ptr: vx_kernel
}

impl Kernel {
}

pub struct Node {
    ptr: vx_node,
}

impl Node {
    fn get_parameter_by_index(&self, index: u32) -> Result<NodeParam> {
        unsafe {
            let ptr = vxGetParameterByIndex(self.ptr, index);
                        let res =  vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(NodeParam{ptr})
        }
    }

    fn set_parameter_by_index(&self, index: usize, reference: &Reference) -> Result<()> {
        unsafe {
            let res = vxSetParameterByIndex(self.ptr, index as u32, reference.to_ref());
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(())
        }
    }
}

pub struct NodeParam {
    ptr: vx_parameter
}

impl NodeParam {
}

pub struct NodeOutputImage {
    node: Rc<Node>,
    param_index: usize,
}

impl InputImage for NodeOutputImage {
    fn set_input_image(&self, graph: &mut Graph, node: &Node, param_index: usize) -> Result<()> {
        let image = graph.create_virtual_image(0, 0, VX_DF_IMAGE_VIRT).unwrap();

        self.node.set_parameter_by_index(self.param_index, &image).unwrap();

        node.set_parameter_by_index(param_index, &image).unwrap();
        Ok(())
    }
}

pub struct GraphInput {
    image: Image,
}

impl InputImage for GraphInput {
    fn set_input_image(&self, graph: &mut Graph, node: &Node, param_index: usize) -> Result<()> {
        let param = node.get_parameter_by_index(param_index as u32).unwrap();
        let graph_index = graph.add_parameter(&param).unwrap();

        graph.set_parameter_by_index(graph_index, &self.image).unwrap();

        node.set_parameter_by_index(param_index, &self.image).unwrap();
        Ok(())
    }
}

