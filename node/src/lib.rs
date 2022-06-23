#[macro_use]
extern crate napi_derive;

use napi::{Env, Error, Result, Status};
use speedy_transform::types::{TransformConfig, TransformOutput};
use speedy_transform::web_transform::parser::*;

mod test;

#[napi]
pub fn transform_babel_import(
  env: Env,
  code: String,
  config: TransformConfig,
  filename: Option<String>,
  target: Option<String>,
) -> Result<TransformOutput> {
  // 占位行...
  let res = transform(env, code.as_str(), config, filename, target);
  match res {
    Ok(result) => Ok(TransformOutput {
      code: result.code,
      map: result.map,
    }),
    Err(msg) => Err(Error::new(Status::FunctionExpected, msg)),
  }
}
