use crate::utils::{constants::app_name, format::format_number};

#[test]
fn given_number_when_format_number_then_should_return_string() {
    assert_eq!(format_number(42), "42");
}

#[test]
fn given_app_name_when_called_then_should_return_parquet_studio() {
    assert_eq!(app_name(), "Parquet Studio");
}
