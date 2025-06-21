# frozen_string_literal: true

module MicroMcp
  module ToolRegistry
    def self.register_tool(name:, description: nil, &block)
      raise ArgumentError, "block required" unless block

      MicroMcpNative.register_tool(name, description, block)
    end
  end
end
