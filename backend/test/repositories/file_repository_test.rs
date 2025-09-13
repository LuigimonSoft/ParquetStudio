use parquet_studio::{
    models::SchemaField,
    repositories::{FileRepository, LocalFileRepository},
};
use std::fs;

#[test]
fn given_schema_when_create_and_read_then_should_match() {
    let repo = LocalFileRepository;
    let path = std::env::temp_dir().join("test_create.parquet");
    let path_str = path.to_str().unwrap();
    repo.create_file(
        path_str,
        vec![SchemaField {
            name: "ID".into(),
            data_type: "Int64".into(),
        }],
    )
    .unwrap();

    let schema = repo.read_schema(path_str).unwrap();
    assert_eq!(schema[0].name, "ID");
    fs::remove_file(path).unwrap();
}
