# frozen_string_literal: true

module MicroMcp
  module PromptRegistry
    def self.register_prompt(name:, description: nil, arguments: nil, &block)
      raise ArgumentError, "block required" unless block

      MicroMcpNative.register_prompt(name, description, arguments, block)
    end
  end
end
