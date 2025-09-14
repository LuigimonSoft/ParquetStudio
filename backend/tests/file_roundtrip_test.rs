use anyhow::Result;
use parquet_studio::{
    models::SchemaField,
    repositories::{FileRepository, LocalFileRepository},
    services::ParquetService,
};
use tempfile::tempdir;

#[test]
fn given_parquet_file_when_load_and_save_then_should_preserve_schema() -> Result<()> {
    let dir = tempdir()?;
    let input = dir.path().join("input.parquet");
    let output = dir.path().join("output.parquet");

    // setup: create initial file with known schema
    let repo_setup = LocalFileRepository;
    repo_setup.create_file(
        input.to_str().unwrap(),
        vec![
            SchemaField {
                name: "ID".into(),
                data_type: "Int64".into(),
            },
            SchemaField {
                name: "NAME".into(),
                data_type: "Utf8".into(),
            },
        ],
    )?;

    let service = ParquetService::new(LocalFileRepository);

    // load step
    let schema = service.get_schema(input.to_str().unwrap())?;
    assert_eq!(schema.len(), 2);
    assert_eq!(schema[0].name, "ID");

    // save step using normalized types
    let mut out_schema = schema.clone();
    out_schema[0].data_type = "Int64".into();
    out_schema[1].data_type = "Utf8".into();
    service.create_file(output.to_str().unwrap(), out_schema)?;

    // verify saved schema matches loaded schema
    let saved_schema = service.get_schema(output.to_str().unwrap())?;
    assert_eq!(schema, saved_schema);

    Ok(())
}
