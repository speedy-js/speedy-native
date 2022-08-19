use napi::JsFunction;
use serde::Serialize;

#[napi(object)]
#[derive(Debug)]
pub struct TransformOutput {
  pub code: String,
  pub map: Option<String>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct TransformConfig {
  pub remove_use_effect: Option<bool>,
  pub react_runtime: Option<bool>,
  pub babel_import: Option<Vec<BabelImportConfig>>,
  pub code_type: Option<String>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct BabelImportConfig {
  pub from_source: String,
  pub replace_css: Option<ReplaceCssConfig>,
  pub replace_js: Option<ReplaceJsConfig>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct ReplaceJsConfig {
  #[serde(skip_serializing)]
  pub replace_expr: JsFunction,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
  pub transform_to_default_import: Option<bool>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct ReplaceCssConfig {
  pub ignore_style_component: Option<Vec<String>>,
  #[serde(skip_serializing)]
  pub replace_expr: JsFunction,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
}
