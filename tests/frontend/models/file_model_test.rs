use parquetstudio_ui::models::{FileModel, SchemaField};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_test::wasm_bindgen_test;

#[cfg(target_arch = "wasm32")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_schema_fields_when_file_model_created_then_should_hold_schema_values() {
    let schema = vec![
        SchemaField {
            name: "id".into(),
            data_type: "INT64".into(),
        },
        SchemaField {
            name: "name".into(),
            data_type: "UTF8".into(),
        },
    ];

    let file = FileModel {
        name: "customers.parquet".into(),
        schema: schema.clone(),
    };

    assert_eq!(file.name, "customers.parquet");
    assert_eq!(file.schema, schema);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn given_two_file_models_when_compared_then_should_detect_equality() {
    let schema = vec![SchemaField {
        name: "total".into(),
        data_type: "DOUBLE".into(),
    }];

    let left = FileModel {
        name: "sales.parquet".into(),
        schema: schema.clone(),
    };

    let right = FileModel {
        name: "sales.parquet".into(),
        schema,
    };

    assert_eq!(left, right);
}
