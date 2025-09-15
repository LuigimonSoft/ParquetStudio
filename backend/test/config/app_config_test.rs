use parquet_studio::config::AppConfig;

#[test]
fn given_missing_config_when_load_then_should_return_default() {
    let cfg = AppConfig::load("nonexistent.json");
    assert!(cfg.last_opened_files.is_empty());
}

#[test]
fn given_valid_config_when_save_and_load_then_should_preserve_state() {
    let path = std::env::temp_dir().join("app_config_test.json");
    let config = AppConfig {
        last_opened_files: vec!["file.parquet".into()],
        active_file: Some("file.parquet".into()),
        mcp_basic: Default::default(),
    };
    config.save(path.to_str().unwrap());
    let loaded = AppConfig::load(path.to_str().unwrap());
    assert_eq!(loaded.active_file.as_deref(), Some("file.parquet"));
    std::fs::remove_file(path).ok();
}
