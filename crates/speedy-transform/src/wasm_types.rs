use serde::{Deserialize, Serialize};

pub struct TransformOutput {
  pub code: String,
  pub map: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformConfig {
  pub remove_use_effect: Option<bool>,
  pub react_runtime: Option<bool>,
  pub babel_import: Option<Vec<BabelImportConfig>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BabelImportConfig {
  pub from_source: String,
  pub replace_css: Option<ReplaceCssConfig>,
  pub replace_js: Option<ReplaceJsConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceJsConfig {
  pub replace_expr: String,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
  pub transform_to_default_import: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplaceCssConfig {
  pub ignore_style_component: Option<Vec<String>>,
  pub replace_expr: String,
  pub lower: Option<bool>,
  pub camel2_dash_component_name: Option<bool>,
}
