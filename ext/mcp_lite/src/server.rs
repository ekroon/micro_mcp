use rust_mcp_sdk::{
    mcp_server::{server_runtime, ServerHandler, ServerRuntime},
    schema::{
        schema_utils::CallToolError, CallToolRequest, CallToolResult, Implementation,
        InitializeResult, ListToolsRequest, ListToolsResult, RpcError, ServerCapabilities,
        ServerCapabilitiesTools, LATEST_PROTOCOL_VERSION, Tool, ToolInputSchema,
    },
    McpServer, StdioTransport, TransportOptions,
};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};
use async_trait::async_trait;
use std::sync::OnceLock;
use std::collections::HashMap;

use crate::utils::nogvl;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

#[derive(Clone)]
struct ToolEntry {
    tool: Tool,
    handler: fn(CallToolRequest) -> Result<CallToolResult, CallToolError>,
}

static TOOLS: OnceLock<HashMap<String, ToolEntry>> = OnceLock::new();

fn tools() -> &'static HashMap<String, ToolEntry> {
    TOOLS.get_or_init(|| {
        let mut map = HashMap::new();
        map.insert(
            SayHelloTool::tool_name(),
            ToolEntry {
                tool: SayHelloTool::tool(),
                handler: say_hello_handler,
            },
        );
        map
    })
}

fn say_hello_handler(_: CallToolRequest) -> Result<CallToolResult, CallToolError> {
    Ok(CallToolResult::text_content("Hello World!".to_string(), None))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SayHelloTool {}

impl SayHelloTool {
    pub fn tool_name() -> String {
        "say_hello_world".to_string()
    }

    pub fn tool() -> Tool {
        Tool {
            annotations: None,
            description: Some("Prints \"Hello World!\" message".to_string()),
            input_schema: ToolInputSchema::new(Vec::new(), None),
            name: Self::tool_name(),
        }
    }
}

pub struct MyServerHandler;

#[async_trait]
impl ServerHandler for MyServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        let tools = tools().values().map(|t| t.tool.clone()).collect();
        Ok(ListToolsResult {
            tools,
            meta: None,
            next_cursor: None,
        })
    }

    async fn handle_call_tool_request(
        &self,
        request: CallToolRequest,
        _runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        match tools().get(request.tool_name()) {
            Some(entry) => (entry.handler)(request),
            None => Err(CallToolError::unknown_tool(request.tool_name().to_string())),
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

#[cfg(test)]
mod tests {
    use rust_mcp_sdk::{
        mcp_client::client_runtime,
        schema::{CallToolRequestParams, ClientCapabilities, Implementation, InitializeRequestParams, LATEST_PROTOCOL_VERSION},
        McpClient, StdioTransport, TransportOptions,
    };
    use async_trait::async_trait;

    struct TestClientHandler;
    #[async_trait]
    impl rust_mcp_sdk::mcp_client::ClientHandler for TestClientHandler {}

    #[tokio::test]
    async fn hello_world_tool_works() {
        let transport = StdioTransport::create_with_server_launch(
            "ruby",
            vec![
                "-I".into(),
                "../../lib".into(),
                "-e".into(),
                "require 'mcp_lite'; McpLite.start_server".into(),
            ],
            None,
            TransportOptions::default(),
        )
        .unwrap();

        let client_details = InitializeRequestParams {
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test-client".into(),
                version: "0.1.0".into(),
            },
            protocol_version: LATEST_PROTOCOL_VERSION.into(),
        };

        let client = client_runtime::create_client(client_details, transport, TestClientHandler);

        client.clone().start().await.unwrap();

        let tools = client.list_tools(None).await.unwrap();
        assert_eq!(tools.tools.len(), 1);
        assert_eq!(tools.tools[0].name, "say_hello_world");

        let result = client
            .call_tool(CallToolRequestParams { name: "say_hello_world".into(), arguments: None })
            .await
            .unwrap();
        let text = result.content[0].as_text_content().unwrap().text.clone();
        assert_eq!(text, "Hello World!");
    }
}

