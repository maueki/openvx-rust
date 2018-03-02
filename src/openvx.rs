
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

error_chain! {
    errors {
        ErrorNotImplemented {
        }
    }
}

pub struct Context {
    ptr: vx_context
}

fn convert_error(err: vx_status) -> ErrorKind {
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

    pub fn get_kernel_by_enum(&self, kernel: vx_enum) -> Result<Kernel> {
        unsafe {
            let ptr = vxGetKernelByEnum(self.ptr, kernel);
            Ok(Kernel{ptr})
        }
    }

}

pub struct Graph {
    ptr: vx_graph
}

impl Graph {
    pub fn create_virtual_image(&self, width: u32, height: u32, color: vx_df_image) -> Result<Image> {
        unsafe {
            let ptr = vxCreateVirtualImage(self.ptr, width, height, color);
            Ok(Image{ptr})
        }
    }
}

pub struct Image {
    ptr: vx_image
}

impl Image {
}

pub struct Kernel {
    ptr: vx_kernel
}

impl Kernel {
}
