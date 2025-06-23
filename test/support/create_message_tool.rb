# frozen_string_literal: true

TR = MicroMcp::ToolRegistry
S = MicroMcp::Schema

TR.register_tool(
  name: "create_message_text",
  description: "invokes runtime.create_message"
) do |_args, runtime|
  result = runtime.create_message(
    {
      "messages" => [
        {"role" => "user", "content" => {"type" => "text", "text" => "What is the capital of France?"}}
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
  result["content"]["text"]
end

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
  result["content"]["text"]
end

TR.register_tool(
  name: "create_message_debug",
  description: "debug version of create_message",
  arguments: S.object(
    question: S.string("Question for the assistant").required
  )
) do |args, runtime|
  # Let's try to debug what's happening

  # First, let's check what args looks like
  puts "Args: #{args.inspect}"
  puts "Args class: #{args.class}"
  puts "Question: #{args["question"].inspect}"

  # Create the message hash step by step
  message_hash = {
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

  puts "Message hash: #{message_hash.inspect}"

  # Try to convert to JSON to see if that's the issue
  require "json"
  json_str = JSON.generate(message_hash)
  puts "JSON string: #{json_str}"

  # Now try the actual call
  result = runtime.create_message(message_hash)
  result["content"]["text"]
rescue => e
  puts "Error: #{e.class}: #{e.message}"
  puts "Backtrace: #{e.backtrace.first(5).join("\n")}"
  "Error occurred: #{e.message}"
end

TR.register_tool(
  name: "show_model_info",
  description: "Shows the model and full response information",
  arguments: S.object(
    question: S.string("Question for the assistant").required
  )
) do |args, runtime|
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

  # Return both the response and model info
  response_text = result["content"]["text"]
  model_used = result["model"] || "unknown"
  stop_reason = result["stopReason"] || "unknown"

  "Response: #{response_text}\n\nModel: #{model_used}\nStop Reason: #{stop_reason}"
end
