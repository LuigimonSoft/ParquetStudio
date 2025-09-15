use crate::services::mcp_service::start_mcp_server;
use futures::executor::block_on;

#[test]
fn given_call_when_start_mcp_server_then_should_return_error() {
    let result = block_on(start_mcp_server());
    assert!(result.is_err());
}
