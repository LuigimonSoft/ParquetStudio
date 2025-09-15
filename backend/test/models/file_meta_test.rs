use parquet_studio::models::FileMeta;

#[test]
fn given_path_and_size_when_create_file_meta_then_should_store_values() {
    let meta = FileMeta {
        path: "data.parquet".into(),
        size: 10,
    };
    assert_eq!(meta.path, "data.parquet");
    assert_eq!(meta.size, 10);
}
