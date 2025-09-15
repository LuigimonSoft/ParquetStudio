use crate::models::SchemaField;
use crate::services::file_service::{create_file, open_file};
use futures::executor::block_on;

#[test]
fn given_path_when_open_file_then_should_return_error() {
    let result = block_on(open_file("test.parquet"));
    assert!(result.is_err());
}

#[test]
fn given_schema_when_create_file_then_should_return_error() {
    let schema = vec![SchemaField {
        name: "id".into(),
        data_type: "Int64".into(),
    }];
    let result = block_on(create_file("new.parquet", schema));
    assert!(result.is_err());
}
