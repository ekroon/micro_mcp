# frozen_string_literal: true

begin
  require_relative "mcp_lite.so"
rescue LoadError
  # The native extension is not built in the test environment.
end
