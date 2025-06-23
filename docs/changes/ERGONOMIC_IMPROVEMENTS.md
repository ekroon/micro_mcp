# MicroMCP Ergonomic Improvements

## Problem

The original MCP tool creation was error-prone due to several issues:

1. **Missing return value handling** - Easy to forget `result["content"]["text"]`
2. **Cryptic error messages** - "no implicit conversion of Hash into String"
3. **Symbol vs String key confusion** - JSON requires string keys
4. **Repetitive boilerplate** - Same patterns repeated in every tool
5. **No validation** - Hard to debug parameter issues

## Solution

We've added several layers of improvements to make tool creation more ergonomic and less error-prone:

### 1. Helper Methods (`runtime_helpers.rb`)

```ruby
# Simple question-answering
runtime.ask_assistant("What is the capital of France?")

# Multi-message conversations
runtime.chat_with_assistant([
  "Let's discuss Ruby",
  {"role" => "assistant", "content" => {"type" => "text", "text" => "I'd love to help!"}},
  "What are the key features?"
])

# Safe wrapper with validation
runtime.safe_create_message(params)
```

### 2. Enhanced Tool Registration (`tool_registry.rb`)

```ruby
# One-liner for simple Q&A tools
TR.register_qa_tool(
  name: "ask",
  description: "Ask a question"
)

# Enhanced registration with error handling
TR.register_assistant_tool(name: "custom", description: "Custom tool") do |args, runtime|
  runtime.ask_assistant(args["question"])
end
```

### 3. Validation System (`validation_helpers.rb`)

- Pre-flight validation of `create_message` parameters
- Clear error messages for common mistakes
- Automatic symbol-to-string key conversion
- Structural validation of message format

## Response Format Handling

The system now properly handles the MCP response format:

```json
{
  "role": "assistant",
  "content": {
    "type": "text",
    "text": "The actual response text"
  },
  "model": "o4-mini",
  "stopReason": "endTurn"
}
```

Helper methods automatically extract `result["content"]["text"]` with proper error handling.

## Before vs After

### Before (Error-prone)
```ruby
TR.register_tool(
  name: "ask",
  arguments: S.object(question: S.string.required)
) do |args, runtime|
  result = runtime.create_message({
    "messages" => [
      {"role" => "user", "content" => {"type" => "text", "text" => args["question"]}}
    ],
    "modelPreferences" => {
      "hints" => [{"name" => "o4-mini"}],
      "intelligencePriority" => 0.8,
      "speedPriority" => 0.5
    },
    "systemPrompt" => "You are a helpful assistant.",
    "maxTokens" => 100
  })
  result["content"]["text"]  # Easy to forget!
end
```

### After (Simple)
```ruby
# One-liner approach
TR.register_qa_tool(name: "ask", description: "Ask a question")

# Or with customization
TR.register_assistant_tool(name: "ask", description: "Ask a question") do |args, runtime|
  runtime.ask_assistant(args["question"])
end
```

## Error Handling

The new system provides:

1. **Pre-validation** - Catches errors before sending to MCP
2. **Clear error messages** - Explains what went wrong
3. **Automatic error recovery** - Handles common formatting issues
4. **Debug mode** - Set `ENV['MCP_DEBUG']` for detailed logging

## Backward Compatibility

All existing tools continue to work unchanged. The improvements are additive.

## Usage Examples

See `test/support/ergonomic_test.rb` and `test/support/ergonomic_examples.rb` for comprehensive examples.

## Key Benefits

1. ✅ **Reduced boilerplate** - Simple tools are one-liners
2. ✅ **Better error messages** - Clear validation feedback
3. ✅ **Automatic result extraction** - No more forgetting return values
4. ✅ **Symbol key safety** - Automatic string conversion
5. ✅ **Validation** - Catch issues before they cause problems
6. ✅ **Backward compatible** - Existing code keeps working
