module RuntimeStore
  class << self
    attr_accessor :captured_runtime
  end
end

MicroMcp::ToolRegistry.register_tool(
  name: "capture_runtime",
  description: "captures runtime"
) do |runtime|
  RuntimeStore.captured_runtime = runtime
  runtime.is_initialized.to_s
end

MicroMcp::ToolRegistry.register_tool(
  name: "use_captured_runtime",
  description: "uses captured runtime"
) do |_|
  RuntimeStore.captured_runtime.is_initialized.to_s
end
