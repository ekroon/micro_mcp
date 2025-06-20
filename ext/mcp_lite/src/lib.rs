use magnus::{function, prelude::*, Error, Ruby};
use rust_mcp_sdk::{macros::{mcp_tool, JsonSchema}, mcp_server::{server_runtime, ServerHandler, ServerRuntime}, schema::{schema_utils::CallToolError, CallToolRequest, CallToolResult, Implementation, InitializeResult, ListToolsRequest, ListToolsResult, RpcError, ServerCapabilities, ServerCapabilitiesTools, LATEST_PROTOCOL_VERSION}, McpServer, StdioTransport, TransportOptions};
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};

use std::sync::OnceLock;
use async_trait::async_trait;

use std::{ffi::c_void, mem::MaybeUninit, ptr::null_mut};

use rb_sys::rb_thread_call_without_gvl;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();

#[mcp_tool(name = "say_hello_world", description = "Prints \"Hello World!\" message")]
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct SayHelloTool {}

pub struct MyServerHandler;

#[async_trait]
impl ServerHandler for MyServerHandler {

    // Handle ListToolsRequest, return list of available tools as ListToolsResult
    async fn handle_list_tools_request(&self, request: ListToolsRequest, runtime: &dyn McpServer) -> Result<ListToolsResult, RpcError> {

        Ok(ListToolsResult {
            tools: vec![SayHelloTool::tool()],
            meta: None,
            next_cursor: None,
        })

    }

    /// Handles requests to call a specific tool.
    async fn handle_call_tool_request( &self, request: CallToolRequest, runtime: &dyn McpServer, ) -> Result<CallToolResult, CallToolError> {

        if request.tool_name() == SayHelloTool::tool_name() {
            Ok(CallToolResult::text_content(
                "Hello World!".to_string(),
                None,
            ))
        } else {
            Err(CallToolError::unknown_tool(request.tool_name().to_string()))
        }

    }
}

fn start_server() -> String {
    // Ensure the Tokio runtime is initialized only once
    let runtime = RUNTIME.get_or_init(|| Runtime::new().expect("Failed to create Tokio runtime"));

    // Use the runtime to block on the async operation
    let _ = nogvl(|| runtime.block_on(async {
        // Simulate an asynchronous operation, e.g., starting a server
        let server_details = InitializeResult {
            // server name and version
            server_info: Implementation {
                name: "Hello World MCP Server".to_string(),
                version: "0.1.0".to_string(),
            },
            capabilities: ServerCapabilities {
                // indicates that server support mcp tools
                tools: Some(ServerCapabilitiesTools { list_changed: None }),
                ..Default::default() // Using default values for other fields
            },
            meta: None,
            instructions: Some("server instructions...".to_string()),
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
        };

        let handler = MyServerHandler {};

        let transport = StdioTransport::new(TransportOptions::default())?;

        let server: ServerRuntime = server_runtime::create_server(server_details, transport, handler);


        server.start().await
    }));

    "Ok".into()
}

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("McpLiteNative")?;
    module.define_singleton_method("start_server", function!(start_server, 0))?;
    Ok(())
}

unsafe extern "C" fn call_without_gvl<F, R>(arg: *mut c_void) -> *mut c_void
where
    F: FnMut() -> R,
    R: Sized,
{
    let arg = arg as *mut (&mut F, &mut MaybeUninit<R>);
    let (func, result) = unsafe { &mut *arg };
    result.write(func());

    null_mut()
}

pub fn nogvl<F, R>(mut func: F) -> R
where
    F: FnMut() -> R,
    R: Sized,
{
    let result = MaybeUninit::uninit();
    let arg_ptr = &(&mut func, &result) as *const _ as *mut c_void;

    unsafe {
        rb_thread_call_without_gvl(Some(call_without_gvl::<F, R>), arg_ptr, None, null_mut());
        result.assume_init()
    }
}
