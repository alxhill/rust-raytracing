use crate::types::Double;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix {
    values: [[Double; 4]; 4],
}
