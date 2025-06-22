# MicroMcp

MicroMcp is a tiny framework for building [MCP](https://github.com/openai/AIAPI-Protocol) servers in Ruby.  It ships with a Rust extension that handles the low level protocol while your Ruby code focuses on registering tools.

The gem is available on [RubyGems](https://rubygems.org/gems/micro_mcp).

Put your Ruby sources under `lib/micro_mcp` and use `bin/console` for an interactive prompt when experimenting with the API.

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

You can also run the bundled `bin/mcp` script and load a file containing your tool registrations:

```bash
bundle exec bin/mcp path/to/tools.rb
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake test` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/ekroon/micro_mcp.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
