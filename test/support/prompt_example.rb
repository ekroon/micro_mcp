# frozen_string_literal: true

PR = MicroMcp::PromptRegistry

PR.register_prompt(
  name: "greeting",
  description: "Simple greeting prompt",
  arguments: [
    {name: "name", description: "Name to greet"}
  ]
) do |args, runtime|
  runtime.is_initialized
  name = args["name"] || "world"
  [
    {"role" => "user", "content" => {"type" => "text", "text" => "Hello #{name}"}},
    {"role" => "assistant", "content" => {"type" => "text", "text" => "Hi!"}}
  ]
end
