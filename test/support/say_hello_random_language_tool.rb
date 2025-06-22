# frozen_string_literal: true

MicroMcp::ToolRegistry.register_tool(
  name: "say_hello_random_language",
  description: "Say hello in a random language using sampling"
) do |runtime|
  runtime.sample!(
    messages: [
      {role: "user", content: "Say hello in a random language"}
    ]
  )
end
