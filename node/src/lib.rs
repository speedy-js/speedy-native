#[macro_use]
extern crate napi_derive;

use std::{
  panic::{catch_unwind, AssertUnwindSafe},
  sync::Arc,
};

use anyhow::anyhow;
use napi::{Error, Result, Status};
use once_cell::sync::Lazy;
use speedy_macro::*;
use speedy_transform::web_transform::parser::*;
use swc::{
  common::{errors::Handler, FileName},
  config::{IsModule, Options},
  ecmascript::ast::EsVersion,
  try_with_handler,
};
use swc_ecma_parser::{EsConfig, Syntax, TsConfig};
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

// static SOURCE_MAP: Lazy<Arc<swc::common::SourceMap>> = Lazy::new(|| Default::default());
static COMPILER: Lazy<Arc<swc::Compiler>> =
  Lazy::new(|| Arc::new(swc::Compiler::new(Default::default())));

pub fn transform(
  id: String,
  code: String,
  config: TransformConfig,
  filename: Option<String>,
  target: Option<String>,
) {
  let compiler = COMPILER.clone();
  let syntax = if id.ends_with(".ts") || id.ends_with(".tsx") {
    Syntax::Typescript(TsConfig {
      tsx: true,
      decorators: true,
      dts: false,
      ..Default::default()
    })
  } else {
    Syntax::Es(EsConfig {
      jsx: true,
      fn_bind: false,
      decorators: true,
      decorators_before_export: true,
      export_default_from: false,
      import_assertions: false,
      static_blocks: false,
      private_in_object: true,
      allow_super_outside_method: false,
    })
  };

  let src_file = compiler.cm.new_source_file(FileName::Real(id.into()), code);
  let output = try_with_handler(compiler.cm.clone(), false, |handler| {
    //
    let ast = compiler
      .parse_js(
        src_file.clone(),
        handler,
        EsVersion::Es2022,
        syntax,
        IsModule::Bool(true),
        false,
      )
      .expect("parse failed");

    let output = compiler
      .process_js_with_custom_pass(
        src_file,
        Some(ast),
        handler,
        &Options::default(),
        Default::default(),
        Default::default(),
      )
      .unwrap();

    Ok(output)
  })
  .unwrap();

  // Some(output)
}
