#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRequest {
    pub page: usize,
    pub page_size: usize,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: 0,
            page_size: 50,
        }
    }
}
