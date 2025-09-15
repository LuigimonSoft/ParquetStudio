pub struct AppState {
    pub current_file: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self { current_file: None }
    }
}
