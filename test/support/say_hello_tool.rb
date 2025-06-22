# frozen_string_literal: true

MicroMcp::ToolRegistry.register_tool(
  name: "say_hello_world",
  description: "Prints 'Hello World!' message"
) do
  "Hello World!"
end
