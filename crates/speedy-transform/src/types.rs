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
  pub remove_useEffect: Option<bool>,
  pub react_runtime: Option<bool>,
  pub babel_import: Option<Vec<BabelImportConfig>>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct BabelImportConfig {
  pub from_source: String,
  pub replace_css: Option<RepalceCssConfig>,
  pub replace_js: Option<RepalceSpecConfig>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct RepalceSpecConfig {
  #[serde(skip_serializing)]
  pub replace_expr: JsFunction,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
  pub transform_to_default_import: Option<bool>,
}

#[napi(object)]
#[derive(Serialize)]
pub struct RepalceCssConfig {
  pub ignore_style_component: Option<Vec<String>>,
  #[serde(skip_serializing)]
  pub replace_expr: JsFunction,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
}
