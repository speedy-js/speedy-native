#[macro_use]
extern crate napi_derive;

use napi::{Error, Result, Status};
use speedy_macro::*;
use speedy_transform::web_transform::parser::*;
use types::*;

mod test;
mod types;

#[speedydebug]
#[napi]
pub fn transform_babel_import(
  code: String,
  config: TransformConfig,
  filename: Option<String>,
  target: Option<String>,
) -> Result<TransformOutput> {
  // 占位行...
  let config_str = serde_json::to_string(&config).unwrap();
  let rsconfig_res = serde_json::from_str::<speedy_transform::types::TransformConfig>(&config_str);
  match rsconfig_res {
    Ok(rsconfig) => {
      let res = transform(code.as_str(), rsconfig, filename, target);
      match res {
        Ok(result) => Ok(TransformOutput {
          code: result.code,
          map: result.map,
        }),
        Err(msg) => Err(Error::new(Status::FunctionExpected, msg)),
      }
    }
    Err(ex) => Err(Error::new(Status::InvalidArg, ex.to_string())),
  }
}
