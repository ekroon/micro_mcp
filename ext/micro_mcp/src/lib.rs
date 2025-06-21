mod utils;
mod server;

use magnus::{function, prelude::*, Error, Ruby};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("MicroMcpNative")?;
    module.define_singleton_method("start_server", function!(server::start_server, 0))?;
    module.define_singleton_method("register_tool", function!(server::register_tool, 3))?;
    Ok(())
}
