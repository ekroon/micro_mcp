MicroMcp::ToolRegistry.register_tool(
  name: "create_message_error",
  description: "invokes runtime.create_message with invalid params"
) do |_args, runtime|
  runtime.create_message({})
end
