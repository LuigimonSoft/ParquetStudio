use parquet_studio::config::{McpSettings, Prompt};

#[test]
fn given_missing_settings_when_load_then_should_return_default() {
    let settings = McpSettings::load("missing_settings.json");
    assert!(settings.prompts.is_empty());
}

#[test]
fn given_valid_settings_when_save_and_load_then_should_preserve_state() {
    let path = std::env::temp_dir().join("mcp_settings_test.json");
    let settings = McpSettings {
        prompts: vec![Prompt {
            id: "p1".into(),
            description: "".into(),
            template: "t".into(),
        }],
        ..Default::default()
    };
    settings.save(path.to_str().unwrap());
    let loaded = McpSettings::load(path.to_str().unwrap());
    assert_eq!(loaded.prompts.len(), 1);
    std::fs::remove_file(path).ok();
}
