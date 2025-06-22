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
use serde_json::{Map as JsonMap, Value as JsonValue};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
use tokio::runtime::Runtime;

use magnus::{block::Proc, value::ReprValue, Error, Ruby, Value};
use magnus::{typed_data::DataTypeFunctions, TypedData};
use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::nogvl;

static RUNTIME: OnceLock<Runtime> = OnceLock::new();
static SHUTDOWN_FLAG: OnceLock<Arc<AtomicBool>> = OnceLock::new();

fn shutdown_flag() -> &'static Arc<AtomicBool> {
    SHUTDOWN_FLAG.get_or_init(|| Arc::new(AtomicBool::new(false)))
}

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

fn ruby_value_to_json_value(ruby: &Ruby, val: Value) -> Result<JsonValue, Error> {
    let json_str: String = magnus::eval!(ruby, "require 'json'; JSON.generate(obj)", obj = val)?;
    serde_json::from_str(&json_str)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))
}

fn json_value_to_ruby_value(ruby: &Ruby, val: &JsonValue) -> Result<Value, Error> {
    let json_str = serde_json::to_string(val)
        .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?;
    Ok(magnus::eval!(
        ruby,
        "require 'json'; JSON.parse(str)",
        str = json_str
    )?)
}

fn parse_tool_input_schema(json: JsonValue) -> ToolInputSchema {
    if let JsonValue::Object(obj) = json {
        let required = obj
            .get("required")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let properties = obj
            .get("properties")
            .and_then(|v| v.as_object())
            .map(|props| {
                props
                    .iter()
                    .filter_map(|(k, v)| match v {
                        JsonValue::Object(map) => Some((k.clone(), map.clone())),
                        _ => None,
                    })
                    .collect::<HashMap<String, JsonMap<String, JsonValue>>>()
            });

        ToolInputSchema::new(required, properties)
    } else {
        ToolInputSchema::new(Vec::new(), None)
    }
}

#[derive(Clone, TypedData)]
#[magnus(class = "MicroMcp::Runtime", free_immediately, unsafe_generics)]
pub struct RubyMcpServer<'a> {
    inner: Rc<RefCell<Option<&'a dyn McpServer>>>,
}

impl<'a> DataTypeFunctions for RubyMcpServer<'a> {}

// SAFETY: the wrapped reference is only used while valid
unsafe impl<'a> Send for RubyMcpServer<'a> {}

impl<'a> RubyMcpServer<'a> {
    fn new(runtime: &'a dyn McpServer) -> Self {
        Self {
            inner: Rc::new(RefCell::new(Some(runtime))),
        }
    }

    fn invalidate(&self) {
        *self.inner.borrow_mut() = None;
    }

    fn runtime(&self) -> Result<&'a dyn McpServer, Error> {
        match *self.inner.borrow() {
            Some(ptr) => Ok(ptr),
            None => {
                let ruby = Ruby::get().unwrap();
                Err(Error::new(
                    ruby.exception_runtime_error(),
                    "McpServer reference is no longer valid",
                ))
            }
        }
    }

    pub fn is_initialized(&self) -> Result<bool, Error> {
        Ok(self.runtime()?.is_initialized())
    }

    pub fn client_supports_sampling(&self) -> Result<Option<bool>, Error> {
        Ok(self.runtime()?.client_supports_sampling())
    }

    pub fn create_message(&self, params: Value) -> Result<Value, Error> {
        let ruby = Ruby::get().unwrap();
        let runtime = self.runtime()?;
        let json_value = ruby_value_to_json_value(&ruby, params)?;
        let request_params: rust_mcp_sdk::schema::CreateMessageRequestParams =
            serde_json::from_value(json_value)
                .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?;

        let runtime_handle = RUNTIME.get().expect("Tokio not initialised");
        let handle = runtime_handle.handle();

        let result = if tokio::runtime::Handle::try_current().is_ok() {
            tokio::task::block_in_place(|| {
                handle.block_on(async { runtime.create_message(request_params).await })
            })
        } else {
            handle.block_on(async { runtime.create_message(request_params).await })
        }
        .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?;

        let json_result = serde_json::to_value(result)
            .map_err(|e| Error::new(ruby.exception_runtime_error(), e.to_string()))?;
        json_value_to_ruby_value(&ruby, &json_result)
    }
}

pub fn register_tool(
    ruby: &Ruby,
    name: String,
    description: Option<String>,
    arg_schema: Option<Value>,
    handler: Proc,
) -> Result<(), Error> {
    let schema = match arg_schema {
        Some(val) => {
            let json = ruby_value_to_json_value(ruby, val)?;
            parse_tool_input_schema(json)
        }
        None => ToolInputSchema::new(Vec::new(), None),
    };

    let tool = Tool {
        annotations: None,
        description,
        input_schema: schema,
        name: name.clone(),
    };

    let handler_fn = RubyHandler(handler);

    let mut map = tools()
        .lock()
        .map_err(|_| Error::new(ruby.exception_runtime_error(), "tools mutex poisoned"))?;
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
            let map = tools().lock().map_err(|_| {
                RpcError::internal_error().with_message("tools mutex poisoned".to_string())
            })?;
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
        runtime: &dyn McpServer,
    ) -> Result<CallToolResult, CallToolError> {
        let map = tools()
            .lock()
            .map_err(|_| CallToolError::new(std::io::Error::other("tools mutex poisoned")))?;
        match map.get(request.tool_name()) {
            Some(entry) => {
                let proc = entry.handler.0;
                let wrapper = RubyMcpServer::new(runtime);
                let args_value = if let Some(map) = &request.params.arguments {
                    let json = JsonValue::Object(map.clone());
                    Some(
                        crate::utils::with_gvl(|| {
                            let ruby = Ruby::get().unwrap();
                            json_value_to_ruby_value(&ruby, &json)
                        })
                        .map_err(|e: Error| {
                            CallToolError::new(std::io::Error::other(e.to_string()))
                        })?,
                    )
                } else {
                    None
                };
                let text_result: Result<String, Error> = crate::utils::with_gvl(|| {
                    let ruby = Ruby::get().unwrap();
                    let args = args_value.unwrap_or_else(|| ruby.qnil().as_value());
                    proc.call::<_, String>((args, wrapper.clone()))
                });
                wrapper.invalidate();
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

    // Reset shutdown flag for new server start
    shutdown_flag().store(false, Ordering::Relaxed);

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

            // Use select! to wait for either server completion or shutdown signal
            tokio::select! {
                result = server.start() => {
                    result
                }
                _ = shutdown_monitor() => {
                    // Server was requested to shutdown
                    Ok(())
                }
                _ = signal_handler() => {
                    // System signal received
                    Ok(())
                }
            }
        })
    });

    "Ok".into()
}

async fn signal_handler() {
    use tokio::signal;

    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {},
        _ = sigterm.recv() => {},
    }
}

async fn shutdown_monitor() {
    let flag = shutdown_flag();
    loop {
        if flag.load(Ordering::Relaxed) {
            break;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

pub fn shutdown_server() -> String {
    shutdown_flag().store(true, Ordering::Relaxed);
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
    use serde_json::json;

    struct TestClientHandler;
    #[async_trait]
    impl rust_mcp_sdk::mcp_client::ClientHandler for TestClientHandler {}

    use rust_mcp_sdk::error::SdkResult;

    #[tokio::test]
    async fn hello_world_tool_works() -> SdkResult<()> {
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
        )?;

        let client_details = InitializeRequestParams {
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test-client".into(),
                version: "0.1.0".into(),
            },
            protocol_version: LATEST_PROTOCOL_VERSION.into(),
        };

        let client = client_runtime::create_client(client_details, transport, TestClientHandler);

        client.clone().start().await?;

        let tools = client.list_tools(None).await?;
        assert_eq!(tools.tools.len(), 1);
        assert_eq!(tools.tools[0].name, "say_hello_world");

        let result = client
            .call_tool(CallToolRequestParams {
                name: "say_hello_world".into(),
                arguments: None,
            })
            .await?;
        let text = result.content[0].as_text_content()?.text.clone();
        assert_eq!(text, "Hello World!");
        Ok(())
    }

    #[tokio::test]
    async fn tools_with_arguments_work() -> SdkResult<()> {
        let transport = StdioTransport::create_with_server_launch(
            "ruby",
            vec![
                "-I".into(),
                "../../lib".into(),
                "../../bin/mcp".into(),
                "../../test/support/argument_tools.rb".into(),
            ],
            None,
            TransportOptions::default(),
        )?;

        let client_details = InitializeRequestParams {
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test-client".into(),
                version: "0.1.0".into(),
            },
            protocol_version: LATEST_PROTOCOL_VERSION.into(),
        };

        let client = client_runtime::create_client(client_details, transport, TestClientHandler);

        client.clone().start().await?;

        let tools = client.list_tools(None).await?;
        assert_eq!(tools.tools.len(), 2);
        assert!(tools.tools.iter().any(|t| t.name == "add_numbers"));
        assert!(tools.tools.iter().any(|t| t.name == "echo_message"));

        let result = client
            .call_tool(CallToolRequestParams {
                name: "add_numbers".into(),
                arguments: Some(
                    [("a".to_string(), json!(5)), ("b".to_string(), json!(7))]
                        .into_iter()
                        .collect(),
                ),
            })
            .await?;
        let text = result.content[0].as_text_content()?.text.clone();
        assert_eq!(text, "12");

        let result = client
            .call_tool(CallToolRequestParams {
                name: "echo_message".into(),
                arguments: Some([("message".to_string(), json!("hi"))].into_iter().collect()),
            })
            .await?;
        let text = result.content[0].as_text_content()?.text.clone();
        assert_eq!(text, "hi");
        Ok(())
    }

    #[tokio::test]
    async fn runtime_lifetime_enforced() -> SdkResult<()> {
        let transport = StdioTransport::create_with_server_launch(
            "ruby",
            vec![
                "-I".into(),
                "../../lib".into(),
                "../../bin/mcp".into(),
                "../../test/support/runtime_lifetime_tool.rb".into(),
            ],
            None,
            TransportOptions::default(),
        )?;

        let client_details = InitializeRequestParams {
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test-client".into(),
                version: "0.1.0".into(),
            },
            protocol_version: LATEST_PROTOCOL_VERSION.into(),
        };

        let client = client_runtime::create_client(client_details, transport, TestClientHandler);

        client.clone().start().await?;

        let tools = client.list_tools(None).await?;
        assert_eq!(tools.tools.len(), 2);

        // first call stores the runtime
        let result = client
            .call_tool(CallToolRequestParams {
                name: "capture_runtime".into(),
                arguments: None,
            })
            .await?;
        let text = result.content[0].as_text_content()?.text.clone();
        assert_eq!(text, "true");

        // second call should fail as runtime was invalidated
        let result = client
            .call_tool(CallToolRequestParams {
                name: "use_captured_runtime".into(),
                arguments: None,
            })
            .await?;
        assert!(result.is_error.unwrap_or(false));
        let text = result.content[0].as_text_content()?.text.clone();
        assert!(text.contains("McpServer reference"));

        Ok(())
    }

    #[tokio::test]
    async fn client_supports_sampling_exposed() -> SdkResult<()> {
        let transport = StdioTransport::create_with_server_launch(
            "ruby",
            vec![
                "-I".into(),
                "../../lib".into(),
                "../../bin/mcp".into(),
                "../../test/support/client_capabilities_tool.rb".into(),
            ],
            None,
            TransportOptions::default(),
        )?;

        let client_details = InitializeRequestParams {
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test-client".into(),
                version: "0.1.0".into(),
            },
            protocol_version: LATEST_PROTOCOL_VERSION.into(),
        };

        let client = client_runtime::create_client(client_details, transport, TestClientHandler);

        client.clone().start().await?;

        let result = client
            .call_tool(CallToolRequestParams {
                name: "client_sampling_supported".into(),
                arguments: None,
            })
            .await?;
        let text = result.content[0].as_text_content()?.text.clone();
        assert_eq!(text, "false");

        Ok(())
    }
}
