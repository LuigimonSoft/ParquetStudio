use crate::services::query_service::run_query;
use futures::executor::block_on;

#[test]
fn given_sql_when_run_query_then_should_return_error() {
    let result = block_on(run_query("SELECT 1"));
    assert!(result.is_err());
}
