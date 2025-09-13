use anyhow::Result;
use mockall::mock;
use parquet_studio::{
    application::FileApp, models::SchemaField, repositories::FileRepository,
    services::ParquetService,
};

mock! {
    pub Repository {}
    impl FileRepository for Repository {
        fn read_schema(&self, path: &str) -> Result<Vec<SchemaField>>;
        fn create_file(&self, path: &str, schema: Vec<SchemaField>) -> Result<()>;
    }
}

#[test]
fn given_service_when_get_schema_then_should_return_uppercase() {
    let mut repo = MockRepository::new();
    repo.expect_read_schema().returning(|_| {
        Ok(vec![SchemaField {
            name: "id".into(),
            data_type: "Int64".into(),
        }])
    });

    let service = ParquetService::new(repo);
    let app = FileApp::new(service);
    let schema = app.get_schema("test").unwrap();
    assert_eq!(schema[0].name, "ID");
}

#[test]
fn given_schema_lowercase_when_create_file_then_should_call_service() {
    let mut repo = MockRepository::new();
    repo.expect_create_file()
        .withf(|_, schema| schema[0].name == "ID")
        .returning(|_, _| Ok(()));

    let service = ParquetService::new(repo);
    let app = FileApp::new(service);
    app.create_file(
        "test.parquet",
        vec![SchemaField {
            name: "id".into(),
            data_type: "Int64".into(),
        }],
    )
    .unwrap();
}
