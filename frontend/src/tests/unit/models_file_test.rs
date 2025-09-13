use crate::models::File;

#[test]
fn given_name_when_create_file_then_should_hold_name() {
    let file = File {
        name: "data.parquet".into(),
    };
    assert_eq!(file.name, "data.parquet");
}
