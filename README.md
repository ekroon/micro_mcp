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

### Version Management

The gem version is managed in a single location: `lib/micro_mcp/version.rb`. This ensures consistency across the project.

#### Releasing a New Version

1. **Update the version**: Edit `lib/micro_mcp/version.rb` and change the `VERSION` constant
2. **Update the changelog**: Add the new version details to `CHANGELOG.md`
3. **Test your changes**: Run `bundle exec rake` to ensure all tests pass
4. **Release the gem**: Run `bundle exec rake release`

The release command will:
- Build the gem file
- Create a git tag for the version
- Push commits and tags to GitHub
- Push the `.gem` file to [rubygems.org](https://rubygems.org)

#### Version Status Checking

The repository includes a GitHub Actions workflow that automatically compares the current version in `version.rb` with the latest published version on RubyGems. This helps maintainers track when a new version is ready for release or if versions have diverged during development.

To install this gem onto your local machine, run `bundle exec rake install`.

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ekroon/micro_mcp.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
