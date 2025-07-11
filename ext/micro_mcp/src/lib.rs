mod server;
mod utils;

use magnus::{function, method, prelude::*, Error, Ruby};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let native = ruby.define_module("MicroMcpNative")?;
    native.define_singleton_method("start_server", function!(server::start_server, 0))?;
    native.define_singleton_method("shutdown_server", function!(server::shutdown_server, 0))?;
    native.define_singleton_method("register_tool", function!(server::register_tool, 4))?;
    native.define_singleton_method("register_prompt", function!(server::register_prompt, 4))?;

    let parent = ruby.define_module("MicroMcp")?;
    let class = parent.define_class("Runtime", ruby.class_object())?;
    class.define_method(
        "is_initialized",
        method!(server::RubyMcpServer::is_initialized, 0),
    )?;
    class.define_method(
        "client_supports_sampling",
        method!(server::RubyMcpServer::client_supports_sampling, 0),
    )?;
    class.define_method(
        "create_message",
        method!(server::RubyMcpServer::create_message, 1),
    )?;
    Ok(())
}
