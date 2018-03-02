
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

error_chain! {
    errors {
        ErrorNotImplemented {
        }
    }
}

pub trait Reference {
    fn to_ref(&self) -> vx_reference;
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
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Context{ptr})
        }
    }

    pub fn create_graph(&mut self) -> Result<Graph> {
        unsafe {
            let ptr = vxCreateGraph(self.ptr);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Graph{ptr})
        }
    }

    pub fn get_kernel_by_enum(&self, kernel: vx_kernel_e::Type) -> Result<Kernel> {
        unsafe {
            let ptr = vxGetKernelByEnum(self.ptr, kernel as i32);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Kernel{ptr})
        }
    }

}

pub struct Graph {
    ptr: vx_graph
}

impl Graph {
    pub fn create_virtual_image(&self, width: u32, height: u32, color: vx_df_image_e::Type) -> Result<Image> {
        unsafe {
            let ptr = vxCreateVirtualImage(self.ptr, width, height, color);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Image{ptr})
        }
    }

    pub fn create_generic_node(&self, kernel: &Kernel) -> Result<Node> {
        unsafe {
            let ptr = vxCreateGenericNode(self.ptr, kernel.ptr);
            let res =  vxGetStatus(ptr as vx_reference);
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(Node{ptr})
        }
    }

    pub fn add_parameter(&self, param: &NodeParam) -> Result<()> {
        unsafe {
            let res = vxAddParameterToGraph(self.ptr, param.ptr);
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(())
        }
    }

    pub fn release(mut self) -> Result<()> {
        unsafe {
            let res = vxReleaseGraph(&mut self.ptr);
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(())
        }
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        unsafe {
            let res = vxReleaseGraph(&mut self.ptr);
            if res != vx_status_e_VX_SUCCESS {
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
            if res != vx_status_e_VX_SUCCESS {
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
    ptr: vx_node
}

impl Node {
    pub fn get_parameter_by_index(&self, index: u32) -> Result<NodeParam> {
        unsafe {
            let ptr = vxGetParameterByIndex(self.ptr, index);
                        let res =  vxGetStatus(ptr as vx_reference);
            if res != vx_status_e_VX_SUCCESS {
                bail!(convert_error(res));
            }

            Ok(NodeParam{ptr})
        }
    }

    pub fn set_parameter_by_index(&self, index: u32, reference: &Reference) -> Result<()> {
        unsafe {
            let res = vxSetParameterByIndex(self.ptr, index, reference.to_ref());
            if res != vx_status_e_VX_SUCCESS {
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
