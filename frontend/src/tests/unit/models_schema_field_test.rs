use crate::models::SchemaField;

#[test]
fn given_name_and_type_when_create_schema_field_then_should_hold_values() {
    let field = SchemaField {
        name: "id".into(),
        data_type: "Int64".into(),
    };
    assert_eq!(field.name, "id");
    assert_eq!(field.data_type, "Int64");
}
