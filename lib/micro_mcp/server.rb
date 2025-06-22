# frozen_string_literal: true

module MicroMcp
  module Server
    def self.start
      thread = Thread.new do
        MicroMcpNative.start_server
      rescue => e
        warn "Error starting server: #{e.message}"
      end

      begin
        thread.join
      rescue Interrupt
        puts "\nShutting down server..."
        MicroMcpNative.shutdown_server
        thread.join
      end

      puts "Server stopped."
    end
  end
end
