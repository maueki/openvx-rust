#![allow(dead_code)]

extern crate vx;
extern crate nnef_parser as np;

#[macro_use]
extern crate error_chain;

error_chain! {
    links {
        Vx(vx::Error, vx::ErrorKind);
    }
}

pub fn doc_to_graph<'a>(context: &'a vx::Context, _doc: &np::Document) -> Result<vx::Graph<'a>> {
    Ok(context.create_graph()?)
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
