# frozen_string_literal: true

module McpLite
  module ToolRegistry
    def self.register_tool(name:, description: nil, &block)
      raise ArgumentError, "block required" unless block

      McpLiteNative.register_tool(name, description, block)
    end
  end
end
