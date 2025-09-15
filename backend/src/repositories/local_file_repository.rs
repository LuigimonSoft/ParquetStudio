use super::FileRepository;
use crate::models::SchemaField;
use anyhow::Result;
use parquet::basic::Type as PhysicalType;
use parquet::file::{
    properties::WriterProperties,
    reader::{FileReader, SerializedFileReader},
    writer::SerializedFileWriter,
};
use parquet::schema::types::Type;
use std::{fs::File, sync::Arc};

pub struct LocalFileRepository;

impl FileRepository for LocalFileRepository {
    fn read_schema(&self, path: &str) -> Result<Vec<SchemaField>> {
        let file = File::open(path)?;
        let reader = SerializedFileReader::new(file)?;
        let schema = reader.metadata().file_metadata().schema_descr();
        let fields = schema
            .columns()
            .iter()
            .map(|c| SchemaField {
                name: c.name().to_string(),
                data_type: format!("{:?}", c.physical_type()),
            })
            .collect();
        Ok(fields)
    }

    fn create_file(&self, path: &str, schema: Vec<SchemaField>) -> Result<()> {
        let fields: Vec<Arc<Type>> = schema
            .into_iter()
            .map(|f| {
                let physical = match f.data_type.as_str() {
                    "Int64" => PhysicalType::INT64,
                    _ => PhysicalType::BYTE_ARRAY,
                };
                Arc::new(
                    Type::primitive_type_builder(&f.name, physical)
                        .build()
                        .unwrap(),
                )
            })
            .collect();
        let schema = Arc::new(
            Type::group_type_builder("schema")
                .with_fields(fields)
                .build()?,
        );
        let file = File::create(path)?;
        let props = Arc::new(WriterProperties::builder().build());
        let writer = SerializedFileWriter::new(file, schema, props)?;
        writer.close()?;
        Ok(())
    }
}
