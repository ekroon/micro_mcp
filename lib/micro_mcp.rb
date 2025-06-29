# frozen_string_literal: true

require_relative "micro_mcp/version"
require_relative "micro_mcp/micro_mcp"
require_relative "micro_mcp/schema"
require_relative "micro_mcp/tool_registry"
require_relative "micro_mcp/server"
require_relative "micro_mcp/runtime_helpers"
require_relative "micro_mcp/validation_helpers"

module MicroMcp
  class Error < StandardError; end
  # Your code goes here...

  def self.start_server
    Server.start
  end
end
