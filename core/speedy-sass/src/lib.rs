use crate::bindings::context::Context;
use crate::options::Options;
use std::path::Path;

mod bindings;
mod options;
mod sass_clib;
mod test;

/// Takes a file path and compiles it with the options given
pub fn compile_file<P: AsRef<Path>>(path: P, options: Options) -> Result<String, String> {
  let mut context = Context::new_file(path)?;
  context.set_options(options);
  context.compile()
}

/// Takes a string and compiles it with the options given
pub fn compile_string(content: &str, options: Options) -> Result<String, String> {
  let mut context = Context::new_data(content);
  context.set_options(options);
  context.compile()
}
