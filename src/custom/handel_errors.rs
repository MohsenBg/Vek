use std::fmt::Debug;

#[derive(Debug)]
pub enum VekError {
    Allocating(&'static str),
    ZeroType(&'static str),
    OverFlow(&'static str),
}

pub fn unwraper<U, E: Debug>(data: std::result::Result<U, E>, error_type: VekError) -> U {
    match data {
        Err(e) => {
            match error_type {
                VekError::Allocating(msg) => {
                    eprintln!("ALLOCATING Error\n{}", msg);
                }
                VekError::ZeroType(msg) => {
                    eprintln!("ZERO_TYPE\n{}", msg)
                }
                VekError::OverFlow(msg) => {
                    eprintln!("OVER_FLOW\n{}", msg)
                }
            }
            panic!("{:?}", e);
            //std::process::exit(1)
        }
        Ok(content) => content,
    }
}
