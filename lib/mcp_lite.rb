# frozen_string_literal: true

require_relative "mcp_lite/version"
require_relative "mcp_lite/mcp_lite"
require_relative "mcp_lite/server"

module McpLite
  class Error < StandardError; end
  # Your code goes here...

  def self.start_server
    Server.start
  end
end
