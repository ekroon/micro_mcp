# frozen_string_literal: true

require_relative "mcp_lite/version"
require_relative "mcp_lite/mcp_lite"

module McpLite
  class Error < StandardError; end
  # Your code goes here...

  def self.start_server
    server_thread = Thread.new do
      begin
      McpLiteNative.start_server
      rescue StandardError => e
      puts "Error starting server: #{e.message}"
      end
    end

    begin
      server_thread.join
    rescue Interrupt
      puts "\nShutting down server..."
      server_thread.kill
      # Add any cleanup logic here if needed
    end

    puts "Server stopped."
    Process.kill('KILL', Process.pid)
  end
end
