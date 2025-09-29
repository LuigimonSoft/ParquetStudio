#[cfg(target_arch = "wasm32")]
#[path = "frontend/app_test.rs"]
mod app;
#[path = "frontend/components/mod.rs"]
mod components;
#[path = "frontend/models/mod.rs"]
mod models;
