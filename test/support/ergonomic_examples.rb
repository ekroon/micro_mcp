# frozen_string_literal: true

require_relative "../../lib/micro_mcp"

TR = MicroMcp::ToolRegistry
S = MicroMcp::Schema

# Example 1: Using the new simplified QA tool registration
TR.register_qa_tool(
  name: "simple_qa",
  description: "Ask a simple question to the assistant"
)

# Example 2: Using the enhanced assistant tool registration with custom logic
TR.register_assistant_tool(
  name: "enhanced_qa",
  description: "Ask a question with custom processing"
) do |args, runtime|
  # The runtime now has helper methods available
  answer = runtime.ask_assistant(
    "Please answer this question concisely: #{args["question"]}",
    system_prompt: "You are a concise and helpful assistant.",
    max_tokens: 150
  )

  # Custom post-processing
  "Answer: #{answer}"
end

# Example 3: Using the chat helper for multi-message conversations
TR.register_assistant_tool(
  name: "chat_example",
  description: "Example of multi-message chat",
  question_param: "topic"
) do |args, runtime|
  messages = [
    "Let's discuss #{args["topic"]}",
    {"role" => "assistant", "content" => {"type" => "text", "text" => "I'd be happy to discuss that topic with you."}},
    "What are the key points I should know?"
  ]

  runtime.chat_with_assistant(
    messages,
    system_prompt: "You are an expert educator.",
    max_tokens: 200
  )
end

# Example 4: Traditional way (still supported) - showing the before/after
TR.register_tool(
  name: "traditional_way",
  description: "The old way of doing things (still works)",
  arguments: S.object(
    question: S.string("Question for the assistant").required
  )
) do |args, runtime|
  # Manual way - more verbose and error-prone
  result = runtime.create_message(
    {
      "messages" => [
        {
          "role" => "user",
          "content" => {"type" => "text", "text" => args["question"]}
        }
      ],
      "modelPreferences" => {
        "hints" => [{"name" => "o4-mini"}],
        "intelligencePriority" => 0.8,
        "speedPriority" => 0.5
      },
      "systemPrompt" => "You are a helpful assistant.",
      "maxTokens" => 100
    }
  )
  result["content"]["text"]  # Easy to forget this line!
end
