MicroMcp::ToolRegistry.register_tool(
  name: "create_message_text",
  description: "invokes runtime.create_message",
  arguments: MicroMcp::Schema.object(
    question: MicroMcp::Schema.string("question to ask").required
  )
) do |args, runtime|
  result = runtime.create_message(
    {
      "messages" => [
        {"role" => "user", "content" => {"type" => "text", "text" => args["question"]}}
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
