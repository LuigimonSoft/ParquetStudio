use parquet_studio::models::PageResponse;
use serde_json::json;

#[test]
fn given_items_when_serialize_then_should_return_json() {
    let response = PageResponse {
        items: vec![1, 2],
        total: 2,
    };
    let json_str = serde_json::to_string(&response).unwrap();
    assert_eq!(json_str, json!({"items": [1, 2], "total": 2}).to_string());
}
