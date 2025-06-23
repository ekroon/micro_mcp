# frozen_string_literal: true

require_relative "validation_helpers"

module MicroMcp
  module RuntimeHelpers
    # Helper method to make create_message calls more ergonomic
    # Automatically extracts the text content from the response
    def ask_assistant(question, system_prompt: "You are a helpful assistant.", max_tokens: 100, model_hints: ["o4-mini"])
      params = {
        "messages" => [
          {
            "role" => "user",
            "content" => {"type" => "text", "text" => question}
          }
        ],
        "modelPreferences" => {
          "hints" => model_hints.map { |name| {"name" => name} },
          "intelligencePriority" => 0.8,
          "speedPriority" => 0.5
        },
        "systemPrompt" => system_prompt,
        "maxTokens" => max_tokens
      }

      # Validate before sending
      errors = ValidationHelpers.validate_create_message_params(params)
      if errors.any?
        raise ArgumentError, "Invalid create_message parameters: #{errors.join(", ")}"
      end

      result = create_message(params)

      # Automatically extract the text content with error handling
      # Response format: { "role": "assistant", "content": { "type": "text", "text": "..." }, "model": "...", "stopReason": "..." }
      if result.is_a?(Hash) && result.dig("content", "text")
        result["content"]["text"]
      else
        raise "Unexpected response format from create_message: #{result.inspect}"
      end
    end

    # More advanced helper that handles different message types
    def chat_with_assistant(messages, system_prompt: "You are a helpful assistant.", max_tokens: 100, model_hints: ["o4-mini"])
      # Normalize messages to the expected format
      normalized_messages = messages.map do |msg|
        case msg
        when String
          {"role" => "user", "content" => {"type" => "text", "text" => msg}}
        when Hash
          # Ensure string keys and validate structure
          normalized = ValidationHelpers.stringify_keys(msg)
          unless normalized.key?("role") && normalized.key?("content")
            raise ArgumentError, "Message hash must have 'role' and 'content' keys: #{msg.inspect}"
          end
          normalized
        else
          raise ArgumentError, "Messages must be strings or hashes, got: #{msg.class}"
        end
      end

      params = {
        "messages" => normalized_messages,
        "modelPreferences" => {
          "hints" => model_hints.map { |name| {"name" => name} },
          "intelligencePriority" => 0.8,
          "speedPriority" => 0.5
        },
        "systemPrompt" => system_prompt,
        "maxTokens" => max_tokens
      }

      # Validate before sending
      errors = ValidationHelpers.validate_create_message_params(params)
      if errors.any?
        raise ArgumentError, "Invalid create_message parameters: #{errors.join(", ")}"
      end

      result = create_message(params)

      # Automatically extract the text content with error handling
      # Response format: { "role": "assistant", "content": { "type": "text", "text": "..." }, "model": "...", "stopReason": "..." }
      if result.is_a?(Hash) && result.dig("content", "text")
        result["content"]["text"]
      else
        raise "Unexpected response format from create_message: #{result.inspect}"
      end
    end

    # Safe wrapper around create_message with validation
    def safe_create_message(params)
      # Ensure all keys are strings
      safe_params = ValidationHelpers.stringify_keys(params)

      # Validate
      errors = ValidationHelpers.validate_create_message_params(safe_params)
      if errors.any?
        raise ArgumentError, "Invalid create_message parameters: #{errors.join(", ")}"
      end

      create_message(safe_params)
    end
  end
end
