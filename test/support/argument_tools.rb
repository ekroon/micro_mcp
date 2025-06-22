# frozen_string_literal: true

# Aliases for cleaner syntax
TR = MicroMcp::ToolRegistry
S = MicroMcp::Schema

TR.register_tool(
  name: "add_numbers",
  description: "Adds two integers",
  arguments: S.object(
    a: S.integer("First integer to add").required,
    b: S.integer("Second integer to add").required
  )
) do |args, _runtime|
  (args["a"] + args["b"]).to_s
end

TR.register_tool(
  name: "echo_message",
  description: "Echoes the provided message",
  arguments: S.object(
    message: S.string("The message to echo back").required
  )
) do |args, _runtime|
  args["message"]
end
