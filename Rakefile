# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"

Minitest::TestTask.create

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("mcp_lite.gemspec")

RbSys::ExtensionTask.new("mcp_lite", GEMSPEC) do |ext|
  ext.lib_dir = "lib/mcp_lite"
end

# Add Rust test task
desc "Run Rust tests"
task :test_rust do
  Dir.chdir("ext/mcp_lite") do
    sh "cargo test --lib"
  end
end

# Add Rust lint task
desc "Run Rust linting with clippy"
task :lint_rust do
  Dir.chdir("ext/mcp_lite") do
    sh "cargo clippy -- -D warnings"
  end
end

task default: %i[compile test test_rust lint_rust standard]
