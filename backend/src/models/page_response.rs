use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
}
