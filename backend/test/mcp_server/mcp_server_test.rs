use parquet_studio::mcp_server::McpServer;

#[test]
fn given_server_when_start_then_should_not_panic() {
    let server = McpServer;
    server.start();
}
