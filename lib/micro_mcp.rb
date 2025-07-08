# frozen_string_literal: true

require_relative "micro_mcp/version"
ruby_version = RUBY_VERSION[/\d+\.\d+/].to_s
begin
  require_relative "micro_mcp/micro_mcp"
rescue LoadError
  begin
    require_relative "micro_mcp/#{ruby_version}/micro_mcp"
  rescue LoadError
    raise LoadError, "No native extension found for Ruby #{ruby_version}"
  end
end
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
