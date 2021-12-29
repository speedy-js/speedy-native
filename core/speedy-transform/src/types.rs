#[derive(Debug)]
pub struct TransformConfig {
  pub react_runtime: Option<bool>,
  pub babel_import: Option<Vec<BabelImportConfig>>,
}

#[derive(Debug)]
pub struct BabelImportConfig {
  pub from_source: String,
  pub replace_css: Option<RepalceCssConfig>,
  pub replace_js: Option<RepalceSpecConfig>,
}

#[derive(Debug)]
pub struct RepalceSpecConfig {
  pub replace_expr: String,
  pub ignore_es_component: Option<Vec<String>>,
  pub lower: Option<bool>,
}

#[derive(Debug)]
pub struct RepalceCssConfig {
  pub ignore_style_component: Option<Vec<String>>,
  pub replace_expr: String,
  pub lower: Option<bool>,
}
