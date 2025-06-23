#!/usr/bin/env ruby
# Test script to verify gem functionality

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "micro_mcp"

puts "✓ MicroMcp gem loaded successfully"
puts "✓ Version: #{MicroMcp::VERSION}"

# Test basic tool registration (similar to what would be tested in CI)
begin
  MicroMcp::ToolRegistry.register_tool(name: "test_tool") do
    "Hello from test tool!"
  end
  puts "✓ Tool registration works"
rescue => e
  puts "❌ Tool registration failed: #{e.message}"
  exit 1
end

puts "✅ All basic functionality tests passed"
