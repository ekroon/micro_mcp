MicroMcp::ToolRegistry.register_tool(
  name: "create_message_text",
  description: "invokes runtime.create_message"
) do |_args, runtime|
  result = runtime.create_message(
    {
      "messages" => [
        {"role" => "user", "content" => {"type" => "text", "text" => "What is the capital of France?"}}
      ],
      "modelPreferences" => {
        "hints" => [{"name" => "claude-3-sonnet"}],
        "intelligencePriority" => 0.8,
        "speedPriority" => 0.5
      },
      "systemPrompt" => "You are a helpful assistant.",
      "maxTokens" => 100
    }
  )
  result["content"]["text"]
end
