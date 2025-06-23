# frozen_string_literal: true

require_relative "../../lib/micro_mcp"

TR = MicroMcp::ToolRegistry
S = MicroMcp::Schema

puts "=== Testing New Ergonomic Tools ==="

# Test 1: Simple QA tool (one-liner)
TR.register_qa_tool(
  name: "simple_qa_test",
  description: "Simple question answering tool",
  system_prompt: "You are a helpful geography assistant.",
  max_tokens: 50
)

# Test 2: Enhanced assistant tool with custom logic
TR.register_assistant_tool(
  name: "enhanced_qa_test",
  description: "Enhanced question answering with custom processing"
) do |args, runtime|
  # Using the new ask_assistant helper
  answer = runtime.ask_assistant(
    "Please provide a brief answer to: #{args["question"]}",
    system_prompt: "You are a concise and helpful assistant.",
    max_tokens: 80
  )

  # Custom formatting
  "✓ Answer: #{answer}"
end

# Test 3: Direct helper usage
TR.register_assistant_tool(
  name: "direct_helper_test",
  description: "Direct use of helper methods"
) do |args, runtime|
  # Test the safe_create_message wrapper
  result = runtime.safe_create_message({
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
  })

  # Manual extraction - but the helper validates everything first
  result["content"]["text"]
end

puts "✓ All ergonomic tools registered successfully!"
puts "✓ Tools available: simple_qa_test, enhanced_qa_test, direct_helper_test"
