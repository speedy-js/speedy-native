use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(Debug)]
pub struct TransformOutput {
  pub code: String,
  pub map: Option<String>,
}

#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct TransformConfig {
  pub react_runtime: Option<bool>,
  pub babel_import: Option<Vec<BabelImportConfig>>,
}

#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct BabelImportConfig {
  pub from_source: String,
  pub replace_css: Option<RepalceCssConfig>,
  pub replace_js: Option<RepalceSpecConfig>,
}

#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RepalceSpecConfig {
  pub replace_expr: String,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
}

#[napi(object)]
#[derive(Debug, Serialize, Deserialize)]
pub struct RepalceCssConfig {
  pub ignore_style_component: Option<Vec<String>>,
  pub replace_expr: String,
  pub lower: Option<bool>,
}
