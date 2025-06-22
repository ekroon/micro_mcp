# frozen_string_literal: true

module MicroMcp
  module ToolRegistry
    def self.register_tool(name:, description: nil, arguments: nil, &block)
      raise ArgumentError, "block required" unless block

      MicroMcpNative.register_tool(name, description, arguments, block)
    end

    # Delegate schema methods to Schema module for convenience
    def self.method_missing(method_name, *args, &block)
      if Schema.respond_to?(method_name)
        Schema.send(method_name, *args, &block)
      else
        super
      end
    end

    def self.respond_to_missing?(method_name, include_private = false)
      Schema.respond_to?(method_name) || super
    end
  end
end
