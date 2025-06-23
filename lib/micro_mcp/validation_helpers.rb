# frozen_string_literal: true

module MicroMcp
  module ValidationHelpers
    # Validate create_message parameters before sending
    def self.validate_create_message_params(params)
      errors = []

      # Check required fields
      errors << "Missing 'messages' field" unless params.key?("messages")
      errors << "Missing 'maxTokens' field" unless params.key?("maxTokens")

      if params["messages"]
        # Validate messages structure
        if params["messages"].is_a?(Array)
          params["messages"].each_with_index do |msg, i|
            unless msg.is_a?(Hash)
              errors << "Message #{i} must be a hash"
              next
            end

            unless msg.key?("role")
              errors << "Message #{i} missing 'role' field"
            end

            unless msg.key?("content")
              errors << "Message #{i} missing 'content' field"
            end

            if msg["content"] && !msg["content"].is_a?(Hash)
              errors << "Message #{i} 'content' must be a hash"
            end
          end
        else
          errors << "'messages' must be an array"
        end
      end

      # Check for common mistakes
      if params.any? { |k, v| k.is_a?(Symbol) }
        errors << "Hash keys must be strings, not symbols. Use string keys throughout."
      end

      errors
    end

    # Helper to convert symbol keys to string keys recursively
    def self.stringify_keys(obj)
      case obj
      when Hash
        obj.transform_keys(&:to_s).transform_values { |v| stringify_keys(v) }
      when Array
        obj.map { |v| stringify_keys(v) }
      else
        obj
      end
    end
  end
end
