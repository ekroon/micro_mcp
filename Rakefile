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

desc "Build binary gem with compiled extension"
task :build_binary do
  # First compile the extension
  Rake::Task[:compile].invoke
  
  # Create a modified gemspec that includes the compiled extension
  spec = GEMSPEC.dup
  
  # Get the current platform
  platform = Gem::Platform.local.to_s
  spec.platform = platform
  
  # Add the compiled extension to the files list
  compiled_files = Dir.glob("lib/**/*.{bundle,so,dylib}")
  spec.files += compiled_files
  
  # Remove the extension requirement since we're including the compiled version
  spec.extensions = []
  
  # Build the gem
  gem_file = "pkg/#{spec.full_name}.gem"
  FileUtils.mkdir_p("pkg")
  
  puts "Building binary gem: #{gem_file}"
  puts "Platform: #{platform}"
  puts "Compiled files: #{compiled_files}"
  
  Gem::Package.build(spec)
  FileUtils.mv("#{spec.full_name}.gem", gem_file)
  
  puts "Built #{gem_file}"
end

task default: %i[compile test test_rust lint_rust standard]
