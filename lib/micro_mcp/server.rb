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
        thread.kill
      end

      puts "Server stopped."
      Process.kill("KILL", Process.pid)
    end
  end
end
