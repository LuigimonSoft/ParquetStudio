use parquet_studio::models::PageRequest;

#[test]
fn given_default_when_new_then_should_have_zero_page() {
    let req = PageRequest::default();
    assert_eq!(req.page, 0);
    assert_eq!(req.page_size, 50);
}
