#![allow(dead_code)]

extern crate vx;
extern crate nnef_parser;
extern crate combine;

use nnef_parser as nnef;

#[macro_use]
extern crate error_chain;

error_chain! {
    links {
        Vx(vx::Error, vx::ErrorKind);
    }
}

pub fn doc_to_graph<'a>(context: &'a vx::Context, _doc: &nnef::Document) -> Result<vx::Graph<'a>> {
    Ok(context.create_graph()?)
}


#[cfg(test)]
mod tests {

    use nnef_parser::*;

static ALEXNET: &'static str = r#"
version 1

graph AlexNet( input ) -> ( output )
{
    conv1 = conv(input, kernel1, bias1, padding = [(0,0), (0,0)],
                 border = 'constant', stride = [4, 4], dilation = [1, 1])
    relu1 = relu(conv1)
    pool1 = max_pool(relu1, size = [1, 1, 3, 3], stride = [1, 1, 2, 2],
                     border = 'ignore', padding = [(0,0), (0,0), (0,0), (0,0)])
    kernel2 = variable(shape = [192, 64, 5, 5], label = 'alexnet_v2/conv2/kernel')
    bias2 = variable(shape = [1, 192], label = 'alexnet_v2/conv2/bias')
    conv2 = conv(pool1, kernel2, bias2, padding = [(2,2), (2,2)],
                 border = 'constant', stride = [1, 1], dilation = [1, 1])
    relu2 = relu(conv2)
    pool2 = max_pool(relu2, size = [1, 1, 3, 3], stride = [1, 1, 2, 2],
                     border = 'ignore', padding = [(0,0), (0,0), (0,0), (0,0)])
    kernel3 = variable(shape = [384, 192, 3, 3], label = 'alexnet_v2/conv3/kernel')
    bias3 = variable(shape = [1, 384], label = 'alexnet_v2/conv3/bias')
    conv3 = conv(pool2, kernel3, bias3, padding = [(1,1), (1,1)],
                 border = 'constant', stride = [1, 1], dilation = [1, 1])
    relu3 = relu(conv3)
    kernel4 = variable(shape = [384, 384, 3, 3], label = 'alexnet_v2/conv4/kernel')
    bias4 = variable(shape = [1, 384], label = 'alexnet_v2/conv4/bias')
    conv4 = conv(relu3, kernel4, bias4, padding = [(1,1), (1,1)],
                 border = 'constant', stride = [1, 1], dilation = [1, 1])
    relu4 = relu(conv4)
    kernel5 = variable(shape = [256, 384, 3, 3], label = 'alexnet_v2/conv5/kernel')
    bias5 = variable(shape = [1, 256], label = 'alexnet_v2/conv5/bias')
    conv5 = conv(relu4, kernel5, bias5, padding = [(1,1), (1,1)],
                 border = 'constant', stride = [1, 1], dilation = [1, 1])
    relu5 = relu(conv5)
    pool3 = max_pool(relu5, size = [1, 1, 3, 3], stride = [1, 1, 2, 2],
                     border = 'ignore', padding = [(0,0), (0,0), (0,0), (0,0)])
    kernel6 = variable(shape = [4096, 256, 5, 5], label = 'alexnet_v2/fc6/kernel')
    bias6 = variable(shape = [1, 4096], label = 'alexnet_v2/fc6/bias')
    conv6 = conv(pool3, kernel6, bias6, padding = [(0,0), (0,0)],
                 border = 'constant', stride = [1, 1], dilation = [1, 1])
    relu6 = relu(conv6)
    kernel7 = variable(shape = [4096, 4096, 1, 1], label = 'alexnet_v2/fc7/kernel')
    bias7 = variable(shape = [1, 4096], label = 'alexnet_v2/fc7/bias')
    conv7 = conv(relu6, kernel7, bias7, padding = [(0,0), (0,0)],
                 border = 'constant', stride = [1, 1], dilation = [1, 1])
    relu7 = relu(conv7)
    kernel8 = variable(shape = [1000, 4096, 1, 1], label = 'alexnet_v2/fc8/kernel')
    bias8 = variable(shape = [1, 1000], label = 'alexnet_v2/fc8/bias')
    conv8 = conv(relu7, kernel8, bias8, padding = [(0,0), (0,0)],
                 border = 'constant', stride = [1, 1], dilation = [1, 1])
    output = softmax(conv8)
}
"#;

    #[test]
    fn alexnet_flat_test() {
        match parse_doc(ALEXNET) {
            Ok(_) => (),
            err => {
                println!("{:?}", err);
                panic!("ALEXNET parse failed")
            }
        }
    }
}
