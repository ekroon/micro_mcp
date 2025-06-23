# MicroMcp

MicroMcp is a tiny framework for building [MCP](https://github.com/openai/AIAPI-Protocol) servers in Ruby.  It ships with a Rust extension that handles the low level protocol while your Ruby code focuses on registering tools.

The gem is available on [RubyGems](https://rubygems.org/gems/micro_mcp).

## Installation

Add the gem to your application's Gemfile:

```ruby
gem "micro_mcp"
```

Then execute:

```bash
bundle install
```

Or install it directly with RubyGems:

```bash
gem install micro_mcp
```

## Usage

Define one or more tools and start the server:

```ruby
require "micro_mcp"

 MicroMcp::ToolRegistry.register_tool(name: "say_hello") do
  "Hello World!"
end

# Tools can also accept arguments defined using JSON Schema.
# The arguments hash is provided as the first block parameter.
 MicroMcp::ToolRegistry.register_tool(
  name: "add_numbers",
  description: "Adds two integers",
  arguments: {
    "type" => "object",
    "properties" => {
      "a" => {"type" => "integer"},
      "b" => {"type" => "integer"}
    },
    "required" => ["a", "b"]
  }
) do |args, _runtime|
  (args["a"] + args["b"]).to_s
end

MicroMcp.start_server
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. 

### Cross-Platform Compilation

This gem uses a GitHub Actions workflow for cross-platform compilation, supporting Linux, macOS, and Windows across multiple Ruby versions. The workflow automatically:

- Cross-compiles the Rust extension for all supported platforms
- Tests the compiled gems on their target platforms  
- Publishes to RubyGems on tagged releases
- Creates GitHub releases with compiled gem artifacts

For detailed information about the cross-compilation setup, see [docs/cross-compilation.md](docs/cross-compilation.md).

### Releasing

To release a new version:

1. Update the version number in `lib/micro_mcp/version.rb`
2. Commit your changes
3. Create and push a version tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
4. The GitHub Actions workflow will automatically build, test, and publish the release

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ekroon/micro_mcp.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
