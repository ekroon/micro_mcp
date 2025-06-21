mod utils;
mod server;

use magnus::{function, prelude::*, Error, Ruby};

#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), Error> {
    let module = ruby.define_module("McpLiteNative")?;
    module.define_singleton_method("start_server", function!(server::start_server, 0))?;
    Ok(())
}
