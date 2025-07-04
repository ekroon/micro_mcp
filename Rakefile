# frozen_string_literal: true

require "bundler/gem_tasks"
require "minitest/test_task"

Minitest::TestTask.create

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("micro_mcp.gemspec")

RbSys::ExtensionTask.new("micro_mcp", GEMSPEC) do |ext|
  ext.lib_dir = "lib/micro_mcp"
  ext.cross_compile = true
  ext.cross_platform = %w[x86_64-linux x86_64-darwin arm64-darwin]
end

# Add Rust test task
desc "Run Rust tests"
task :test_rust do
  Dir.chdir("ext/micro_mcp") do
    sh "cargo test --lib"
  end
end

# Add Rust lint task
desc "Run Rust linting with clippy"
task :lint_rust do
  Dir.chdir("ext/micro_mcp") do
    sh "cargo clippy -- -D warnings"
  end
end

task default: %i[compile test test_rust lint_rust standard]
