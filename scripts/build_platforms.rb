#!/usr/bin/env ruby
# frozen_string_literal: true

# Script to build MicroMcp gem for multiple platforms using rb-sys-dock
# This script calls rb-sys-dock for each platform with the specified Ruby versions

PLATFORMS = %w[arm64-darwin x86_64-darwin x86_64-linux].freeze
RUBY_VERSIONS = "3.2,3.3,3.4"

def run_command(command)
  puts "Running: #{command}"
  success = system(command)
  unless success
    puts "ERROR: Command failed: #{command}"
    exit 1
  end
  puts "✓ Command completed successfully\n"
end

def main
  puts "Building MicroMcp gem for multiple platforms..."
  puts "Platforms: #{PLATFORMS.join(', ')}"
  puts "Ruby versions: #{RUBY_VERSIONS}"
  puts

  run_command("bundle exec rake build")

  PLATFORMS.each do |platform|
    puts "=" * 60
    puts "Building for platform: #{platform}"
    puts "=" * 60

    command = "bundle exec rb-sys-dock --platform #{platform} --ruby-versions #{RUBY_VERSIONS} --build"
    run_command(command)
  end

  puts "=" * 60
  puts "All platforms built successfully! 🎉"
  puts "=" * 60
end

if __FILE__ == $PROGRAM_NAME
  main
end
