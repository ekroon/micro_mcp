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

task default: %i[compile test standard]
