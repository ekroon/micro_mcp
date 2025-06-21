use rust_mcp_sdk::{
    macros::{mcp_tool, JsonSchema},
    mcp_server::{server_runtime, ServerHandler, ServerRuntime},
    schema::{
        schema_utils::CallToolError, CallToolRequest, CallToolResult, Implementation,
        InitializeResult, ListToolsRequest, ListToolsResult, RpcError, ServerCapabilities,
        ServerCapabilitiesTools, LATEST_PROTOCOL_VERSION,
    },
    McpServer, StdioTransport, TransportOptions,
};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::sync::OnceLock;

use crate::utils::nogvl;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

#[mcp_tool(name = "say_hello_world", description = "Prints \"Hello World!\" message")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SayHelloTool {}

pub struct MyServerHandler;

#[async_trait]
impl ServerHandler for MyServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        Ok(ListToolsResult {
            tools: vec![SayHelloTool::tool()],
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        if request.tool_name() == SayHelloTool::tool_name() {
            Ok(CallToolResult::text_content("Hello World!".to_string(), None))
        } else {
            Err(CallToolError::unknown_tool(request.tool_name().to_string()))
        }
    }
}

pub fn start_server() -> String {
    let runtime = RUNTIME
        .get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"));

    let _ = nogvl(|| runtime.block_on(async {
        let server_details = InitializeResult {
            server_info: Implementation {
                name: "Hello World MCP Server".to_string(),
                version: "0.1.0".to_string(),
            },
            capabilities: ServerCapabilities {
                tools: Some(ServerCapabilitiesTools { list_changed: None }),
                ..Default::default()
            },
            meta: None,
            instructions: Some("server instructions...".to_string()),
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
        };

        let handler = MyServerHandler {};
        let transport = StdioTransport::new(TransportOptions::default())?;
        let server: ServerRuntime =
            server_runtime::create_server(server_details, transport, handler);

        server.start().await
    }));

    "Ok".into()
}
