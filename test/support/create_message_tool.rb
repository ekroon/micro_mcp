# frozen_string_literal: true

TR = MicroMcp::ToolRegistry
S = MicroMcp::Schema

TR.register_tool(
  name: "create_message_error",
  description: "invokes runtime.create_message with invalid params"
) do |_args, runtime|
  runtime.create_message({})
end

TR.register_tool(
  name: "create_message",
  description: "asks the runtime to create a message",
  arguments: S.object(
    question: S.string("Question for the assistant").required
  )
) do |args, runtime|
  runtime.create_message(
    messages: [
      {
        role: "user",
        content: {type: "text", text: args["question"]}
      }
    ],
    modelPreferences: {
      hints: [{name: "claude-3-sonnet"}],
      intelligencePriority: 0.8,
      speedPriority: 0.5
    },
    systemPrompt: "You are a helpful assistant.",
    maxTokens: 100
  )
end
