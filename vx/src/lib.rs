#![allow(dead_code)]
#![feature(specialization)]

extern crate vx_api;
#[macro_use]
extern crate error_chain;

use vx_api::*;

pub use vx_df_image_e::*;
pub use vx_channel_e::*;
pub use vx_type_e::*;
pub use vx_image_attribute_e::*;

use vx_kernel_e::*;
use vx_status_e::*;
use vx_memory_type_e::*;

use std::mem;
use std::os::raw::c_void;
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

pub trait InputScalar {
    fn set_input_scalar(
        &self,
        graph: &mut Graph,
        node: &Node,
        param_index: usize,
        data_type: vx_type_e::Type,
    ) -> Result<()>;
}

pub trait InputArray {
    fn set_input_scalar(
        &self,
        graph: &mut Graph,
        node: &Node,
        param_index: usize,
        item_type: vx_type_e::Type,
    ) -> Result<()>;
}

pub struct Context {
    pub ptr: vx_context,
}

fn convert_error(err: vx_status) -> ErrorKind {
    println!("!!!! error: {}", err);
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

            Ok(Context { ptr })
        }
    }

    pub fn create_graph(&mut self) -> Result<Graph> {
        unsafe {
            let ptr = vxCreateGraph(self.ptr);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Graph::new(ptr, self))
        }
    }

    pub fn get_kernel_by_enum(&self, kernel: vx_kernel_e::Type) -> Result<Kernel> {
        unsafe {
            let ptr = vxGetKernelByEnum(self.ptr, kernel as i32);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Kernel { ptr })
        }
    }

    pub fn create_image(
        &self,
        width: u32,
        height: u32,
        color: vx_df_image_e::Type,
    ) -> Result<Image> {
        unsafe {
            let ptr = vxCreateImage(self.ptr, width, height, color);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Image { ptr })
        }
    }

    // Unsafe because maybe crash caused by illegal addr
    pub unsafe fn create_image_from_buffer(&self, format: vx_df_image_e::Type,
                                    addr: &vx_imagepatch_addressing_t,
                                    buffer: &[u8]) -> Result<Image> {
        let ptr = vxCreateImageFromHandle(self.ptr, format,
                                          addr as *const _,
                                          &buffer as *const _ as *const *const c_void,
                                          VX_MEMORY_TYPE_HOST as i32);
        let res = vxGetStatus(ptr as vx_reference);
        if res != vx_status_e::VX_SUCCESS {
            bail!(convert_error(res));
        }
        Ok(Image{ptr})
    }

    pub fn create_scalar<T>(&self, data_type: vx_type_e::Type, val: &T) -> Result<Scalar> {
        assert!(data_type > VX_TYPE_INVALID);
        assert!(data_type < VX_TYPE_VENDOR_STRUCT_END);

        unsafe {
            let ptr = vxCreateScalar(self.ptr, data_type as i32, val as *const _ as *const c_void);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Scalar { ptr })
        }
    }

    pub fn create_array(&self, item_type: vx_type_e::Type, capacity: usize) -> Result<Array> {
        assert!(item_type > VX_TYPE_INVALID);
        assert!(item_type < VX_TYPE_VENDOR_STRUCT_END);
        assert!(capacity > 0);

        unsafe {
            let ptr = vxCreateArray(self.ptr, item_type as i32, capacity);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Array { ptr })
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
        Graph {
            ptr: ptr,
            ctx: ctx,
            param_num: 0,
        }
    }

    fn create_virtual_image(
        &self,
        width: u32,
        height: u32,
        color: vx_df_image_e::Type,
    ) -> Result<Image> {
        unsafe {
            let ptr = vxCreateVirtualImage(self.ptr, width, height, color);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Image { ptr })
        }
    }

    fn create_virtual_scalar(&self, data_type: vx_type_e::Type) -> Result<Scalar> {
        unsafe {
            let ptr = vxCreateVirtualScalar(self.ptr, data_type as i32);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Scalar { ptr })
        }
    }

    fn create_virtual_array(&self, item_type: vx_type_e::Type, capacity: usize) -> Result<Array> {
        unsafe {
            let ptr = vxCreateVirtualArray(self.ptr, item_type as i32, capacity);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Array { ptr })
        }
    }

    fn create_generic_node(&self, kernel: &Kernel) -> Result<Node> {
        unsafe {
            let ptr = vxCreateGenericNode(self.ptr, kernel.ptr);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Node { ptr: ptr })
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

        let out = NodeOutputImage {
            node: node.clone(),
            param_index: 1,
        };

        Ok(out)
    }

    pub fn sobel_3x3(&mut self, input: &InputImage) -> Result<(NodeOutputImage, NodeOutputImage)> {

        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_SOBEL_3x3)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        input.set_input_image(self, &node, 0).unwrap();

        let out1 = NodeOutputImage {
            node: node.clone(),
            param_index: 1,
        };
        let out2 = NodeOutputImage {
            node: node.clone(),
            param_index: 2,
        };


        Ok((out1, out2))
    }

    pub fn magnitude(
        &mut self,
        grad_x: &InputImage,
        grad_y: &InputImage,
    ) -> Result<NodeOutputImage> {
        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_MAGNITUDE)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        grad_x.set_input_image(self, &node, 0).unwrap();
        grad_y.set_input_image(self, &node, 1).unwrap();

        let out = NodeOutputImage {
            node: node.clone(),
            param_index: 2,
        };
        Ok(out)
    }

    pub fn channel_extract(
        &mut self,
        input: &InputImage,
        channel: &InputScalar,
    ) -> Result<NodeOutputImage> {
        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_CHANNEL_EXTRACT)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        input.set_input_image(self, &node, 0).unwrap();
        channel
            .set_input_scalar(self, &node, 1, VX_TYPE_ENUM)
            .unwrap();

        let out = NodeOutputImage {
            node: node.clone(),
            param_index: 2,
        };
        Ok(out)
    }

    pub fn median_3x3(&mut self, input: &InputImage) -> Result<NodeOutputImage> {
        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_MEDIAN_3x3)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        input.set_input_image(self, &node, 0).unwrap();

        let out = NodeOutputImage {
            node: node.clone(),
            param_index: 1,
        };
        Ok(out)
    }

    pub fn harris_corners(
        &mut self,
        input: &InputImage,
        strength_thresh: &InputScalar,
        min_distance: &InputScalar,
        sensitivity: &InputScalar,
        gradient_size: &InputScalar,
        block_size: &InputScalar,
    ) -> Result<(NodeOutputArray, NodeOutputScalar)> {
        let kernel = self.ctx.get_kernel_by_enum(VX_KERNEL_HARRIS_CORNERS)?;
        let node = Rc::new(self.create_generic_node(&kernel)?);

        input.set_input_image(self, &node, 0).unwrap();
        strength_thresh
            .set_input_scalar(self, &node, 1, VX_TYPE_FLOAT32)
            .unwrap();
        min_distance
            .set_input_scalar(self, &node, 2, VX_TYPE_FLOAT32)
            .unwrap();
        sensitivity
            .set_input_scalar(self, &node, 3, VX_TYPE_FLOAT32)
            .unwrap();
        gradient_size
            .set_input_scalar(self, &node, 4, VX_TYPE_INT32)
            .unwrap();
        block_size
            .set_input_scalar(self, &node, 5, VX_TYPE_INT32)
            .unwrap();

        let corners = NodeOutputArray {
            node: node.clone(),
            param_index: 6,
            item_type: VX_TYPE_KEYPOINT,
        };

        let num_corners = NodeOutputScalar {
            node: node.clone(),
            param_index: 7,
            data_type: VX_TYPE_SIZE,
        };

        Ok((corners, num_corners))
    }

    pub fn create_input(&self, image: Image) -> Result<GraphInput> {
        Ok(GraphInput { image: image })
    }

    pub fn set_output(&mut self, output: &NodeOutputImage, image: &Image) -> Result<()> {
        let param = output
            .node
            .get_parameter_by_index(output.param_index as u32)
            .unwrap();
        let param_index = self.add_parameter(&param).unwrap();

        self.set_parameter_by_index(param_index, image).unwrap();
        Ok(())
    }

    pub fn set_output_array(&mut self, output: &NodeOutputArray, array: &Array) -> Result<()> {
        let param = output
            .node
            .get_parameter_by_index(output.param_index as u32)
            .unwrap();

        let param_index = self.add_parameter(&param).unwrap();
        self.set_parameter_by_index(param_index, array).unwrap();
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
    pub ptr: vx_image,
}

impl Image {
    pub fn query<T>(&self, attr: vx_image_attribute_e::Type) -> Result<T> {
        unsafe {
            let mut val: T = mem::uninitialized();
            let res = vxQueryImage(
                self.ptr,
                attr as i32,
                &mut val as *mut _ as *mut c_void,
                std::mem::size_of_val(&val),
            );
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(val)
        }
    }
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

pub struct Scalar {
    ptr: vx_scalar,
}

impl Scalar {}

impl Drop for Scalar {
    fn drop(&mut self) {
        unsafe {
            let res = vxReleaseScalar(&mut self.ptr);
            if res != VX_SUCCESS {
                panic!("failed to release scalar");
            }
        }
    }
}

impl Reference for Scalar {
    fn to_ref(&self) -> vx_reference {
        self.ptr as vx_reference
    }
}

pub struct Array {
    ptr: vx_array,
}

impl Array {}

impl Drop for Array {
    fn drop(&mut self) {
        unsafe {
            let res = vxReleaseArray(&mut self.ptr);
            if res != VX_SUCCESS {
                panic!("failed to release array");
            }
        }
    }
}

impl Reference for Array {
    fn to_ref(&self) -> vx_reference {
        self.ptr as vx_reference
    }
}

pub struct Kernel {
    ptr: vx_kernel,
}

impl Kernel {}

pub struct Node {
    ptr: vx_node,
}

impl Node {
    fn get_parameter_by_index(&self, index: u32) -> Result<NodeParam> {
        unsafe {
            let ptr = vxGetParameterByIndex(self.ptr, index);
            let res = vxGetStatus(ptr as vx_reference);
            if res != VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(NodeParam { ptr })
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
    ptr: vx_parameter,
}

impl NodeParam {}

pub struct NodeOutputImage {
    node: Rc<Node>,
    param_index: usize,
}

impl InputImage for NodeOutputImage {
    fn set_input_image(&self, graph: &mut Graph, node: &Node, param_index: usize) -> Result<()> {
        let image = graph.create_virtual_image(0, 0, VX_DF_IMAGE_VIRT).unwrap();

        self.node
            .set_parameter_by_index(self.param_index, &image)
            .unwrap();

        node.set_parameter_by_index(param_index, &image).unwrap();
        Ok(())
    }
}

pub struct NodeOutputScalar {
    node: Rc<Node>,
    param_index: usize,
    data_type: vx_type_e::Type,
}

impl InputScalar for NodeOutputScalar {
    fn set_input_scalar(
        &self,
        graph: &mut Graph,
        node: &Node,
        param_index: usize,
        data_type: vx_type_e::Type,
    ) -> Result<()> {
        assert_eq!(self.data_type, data_type);

        let scalar = graph.create_virtual_scalar(data_type).unwrap();
        self.node
            .set_parameter_by_index(self.param_index, &scalar)
            .unwrap();

        node.set_parameter_by_index(param_index, &scalar).unwrap();
        Ok(())
    }
}

default impl<T> InputScalar for T {
    fn set_input_scalar(
        &self,
        graph: &mut Graph,
        node: &Node,
        param_index: usize,
        data_type: vx_type_e::Type,
    ) -> Result<()> {
        let scalar = graph.ctx.create_scalar(data_type, self).unwrap();
        node.set_parameter_by_index(param_index, &scalar).unwrap();
        Ok(())
    }
}

pub struct NodeOutputArray {
    node: Rc<Node>,
    param_index: usize,
    item_type: vx_type_e::Type,
}

impl InputArray for NodeOutputArray {
    fn set_input_scalar(
        &self,
        graph: &mut Graph,
        node: &Node,
        param_index: usize,
        item_type: vx_type_e::Type,
    ) -> Result<()> {
        assert_eq!(self.item_type, item_type);

        let array = graph.create_virtual_array(item_type, 0).unwrap();
        self.node
            .set_parameter_by_index(self.param_index, &array)
            .unwrap();

        node.set_parameter_by_index(param_index, &array).unwrap();
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

        graph
            .set_parameter_by_index(graph_index, &self.image)
            .unwrap();

        node.set_parameter_by_index(param_index, &self.image)
            .unwrap();
        Ok(())
    }
}
