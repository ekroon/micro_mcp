MicroMcp::ToolRegistry.register_tool(
  name: "client_sampling_supported",
  description: "reports if client supports sampling"
) do |_args, runtime|
  runtime.client_supports_sampling.inspect
end
