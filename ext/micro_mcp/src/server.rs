use async_trait::async_trait;
use rust_mcp_sdk::{
    mcp_server::{server_runtime, ServerHandler, ServerRuntime},
    schema::{
        schema_utils::CallToolError, CallToolRequest, CallToolResult, Implementation,
        InitializeResult, ListToolsRequest, ListToolsResult, RpcError, ServerCapabilities,
        ServerCapabilitiesTools, Tool, ToolInputSchema, LATEST_PROTOCOL_VERSION,
    },
    McpServer, StdioTransport, TransportOptions,
};
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use tokio::runtime::Runtime;

use magnus::{block::Proc, Error, Ruby};

use crate::utils::nogvl;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

type ToolHandler = RubyHandler;

#[derive(Clone)]
struct RubyHandler(Proc);

// SAFETY: We only call the stored Proc while holding the GVL.
unsafe impl Send for RubyHandler {}
unsafe impl Sync for RubyHandler {}

#[derive(Clone)]
struct ToolEntry {
    tool: Tool,
    handler: ToolHandler,
}

static TOOLS: OnceLock<Mutex<HashMap<String, ToolEntry>>> = OnceLock::new();

fn tools() -> &'static Mutex<HashMap<String, ToolEntry>> {
    TOOLS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn register_tool(
    _ruby: &Ruby,
    name: String,
    description: Option<String>,
    handler: Proc,
) -> Result<(), Error> {
    let tool = Tool {
        annotations: None,
        description,
        input_schema: ToolInputSchema::new(Vec::new(), None),
        name: name.clone(),
    };

    let handler_fn = RubyHandler(handler);

    let mut map = tools().lock().unwrap();
    map.insert(
        name,
        ToolEntry {
            tool,
            handler: handler_fn,
        },
    );
    Ok(())
}

pub struct MyServerHandler;

#[async_trait]
impl ServerHandler for MyServerHandler {
    async fn handle_list_tools_request(
        &self,
        _request: ListToolsRequest,
        _runtime: &dyn McpServer,
    ) -> Result<ListToolsResult, RpcError> {
        let tools = {
            let map = tools().lock().unwrap();
            map.values().map(|t| t.tool.clone()).collect()
        };
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
        let map = tools().lock().unwrap();
        match map.get(request.tool_name()) {
            Some(entry) => {
                let proc = entry.handler.0;
                let text_result: Result<String, Error> =
                    crate::utils::with_gvl(|| proc.call::<_, String>(()));
                match text_result {
                    Ok(text) => Ok(CallToolResult::text_content(text, None)),
                    Err(e) => Err(CallToolError::new(std::io::Error::other(e.to_string()))),
                }
            }
            None => Err(CallToolError::unknown_tool(request.tool_name().to_string())),
        }
    }
}

pub fn start_server() -> String {
    let runtime = RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"));

    let _ = nogvl(|| {
        runtime.block_on(async {
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
        })
    });

    "Ok".into()
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use rust_mcp_sdk::{
        mcp_client::client_runtime,
        schema::{
            CallToolRequestParams, ClientCapabilities, Implementation, InitializeRequestParams,
            LATEST_PROTOCOL_VERSION,
        },
        McpClient, StdioTransport, TransportOptions,
    };

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
                "../../bin/mcp".into(),
                "../../test/support/say_hello_tool.rb".into(),
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
            .call_tool(CallToolRequestParams {
                name: "say_hello_world".into(),
                arguments: None,
            })
            .await
            .unwrap();
        let text = result.content[0].as_text_content().unwrap().text.clone();
        assert_eq!(text, "Hello World!");
    }
}
