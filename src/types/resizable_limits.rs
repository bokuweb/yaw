/*
  Field	   Type	       Description
  flags	   varuint1    1 if the maximum field is present, 0 otherwise
  initial  varuint32   initial length (in units of table elements or wasm pages)
  maximum  varuint32?  only present if specified by flags
*/
#[derive(Debug, Clone, PartialEq)]
pub struct ResizableLimits {
    pub initial: u32,
    pub maximum: Option<u32>,
}

impl ResizableLimits {
    pub fn new(initial: u32, maximum: Option<u32>) -> ResizableLimits {
        ResizableLimits { initial, maximum }
    }
}
