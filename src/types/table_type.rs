use super::elem_type::ElemType;
use super::resizable_limits::ResizableLimits;

#[derive(Debug)]
pub struct TableType {
    pub element_type: ElemType,
    pub limits: ResizableLimits,
}
