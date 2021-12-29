#[macro_use]
extern crate napi_derive;

use napi::{Error, Result, Status};
use speedy_transform::web_transform::parser::*;
use types::*;

mod test;
mod types;

#[napi]
pub fn transform_babel_import(
  code: String,
  config: TransformConfig,
  filename: Option<String>,
  target: Option<String>,
) -> Result<TransformOutput> {
  // 占位行...
  let react_runtime = config.react_runtime;

  let mut babel_import: Option<Vec<speedy_transform::types::BabelImportConfig>> = None;

  if let Some(config) = config.babel_import {
    babel_import = Some(
      config
        .into_iter()
        .map(|c| {
          let mut css: Option<speedy_transform::types::RepalceCssConfig> = None;
          if c.replace_css.is_some() {
            let mut ignore_style_component: Option<Vec<String>> = None;
            let na_css = c.replace_css.as_ref().unwrap();
            if na_css.ignore_style_component.is_some() {
              ignore_style_component = na_css.ignore_style_component.clone();
            }
            css = Some(speedy_transform::types::RepalceCssConfig {
              ignore_style_component,
              replace_expr: na_css.replace_expr.clone(),
              lower: na_css.lower,
            });
          }

          let mut es: Option<speedy_transform::types::RepalceSpecConfig> = None;
          if c.replace_js.is_some() {
            let mut ignore_es_component: Option<Vec<String>> = None;
            let na_js = c.replace_js.as_ref().unwrap();
            if na_js.ignore_es_component.is_some() {
              ignore_es_component = na_js.ignore_es_component.clone();
            }
            es = Some(speedy_transform::types::RepalceSpecConfig {
              ignore_es_component,
              replace_expr: na_js.replace_expr.clone(),
              lower: na_js.lower,
            });
          }

          speedy_transform::types::BabelImportConfig {
            from_source: c.from_source,
            replace_css: css,
            replace_js: es,
          }
        })
        .collect::<Vec<speedy_transform::types::BabelImportConfig>>(),
    );
  }

  let rsconfig = speedy_transform::types::TransformConfig {
    react_runtime,
    babel_import,
  };

  let res = transform(code.as_str(), rsconfig, filename, target);

  match res {
    Ok(result) => Ok(TransformOutput {
      code: result.code,
      map: result.map,
    }),
    Err(msg) => Err(Error::new(Status::FunctionExpected, msg)),
  }
}
