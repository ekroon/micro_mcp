# frozen_string_literal: true

MicroMcp::ToolRegistry.register_tool(
  name: "say_hello_random_language",
  description: "Say hello in a random language using sampling"
) do |runtime|
  result = runtime.sample!(
    messages: [
      {
        role: "user",
        content: {type: "text", text: "Say hello in a random language"}
      }
    ],
    maxTokens: 5
  )
  result["content"]["text"]
end
