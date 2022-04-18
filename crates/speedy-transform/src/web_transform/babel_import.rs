use crate::types::TransformConfig;
use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ImportDecl, ImportDefaultSpecifier, ImportSpecifier, ModuleDecl, ModuleItem, Str,
};

struct EsSpec {
  source: String,
  default_spec: String,
}

pub fn transform_style(module: &mut swc_ecma_ast::Module, project_config: &TransformConfig) {
  // let s = serde_json::to_string_pretty(&module).expect("failed to serialize");

  if project_config.babel_import.is_none()
    || project_config.babel_import.as_ref().unwrap().is_empty()
  {
    return;
  }
  let mut specifiers_css = vec![];
  let mut specifiers_es = vec![];
  let mut specifiers_rm_es = vec![];

  let config = project_config.babel_import.as_ref().unwrap();

  for item in &module.body {
    let item_index = module.body.iter().position(|citem| citem == item).unwrap();
    if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
      let source = &*var.src.value;
      if let Some(child_config) = config.iter().find(|&c| c.from_source == source) {
        for specifier in &var.specifiers {
          match specifier {
            ImportSpecifier::Named(ref s) => {
              let ident = s.local.sym.to_string();
              // 替换对应的 css
              if let Some(ref css) = child_config.replace_css {
                let replace_expr = css.replace_expr.as_str();
                let ignore_component = &css.ignore_style_component;
                let need_lower = css.lower.unwrap_or(false);
                let css_ident = if need_lower {
                  ident.to_lowercase()
                } else {
                  ident.clone()
                };
                let mut need_replace = true;
                if let Some(block_list) = ignore_component {
                  need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                }
                if need_replace {
                  let import_css_source =
                    std::string::String::from(replace_expr).replace("{}", css_ident.as_str());
                  specifiers_css.push(import_css_source);
                }
              }
              // 替换对应 spec 的js
              if let Some(ref js_config) = child_config.replace_js {
                let replace_expr = js_config.replace_expr.as_str();
                let ignore_component = &js_config.ignore_es_component;
                let need_lower = js_config.lower.unwrap_or(false);
                let mut js_ident = ident.clone();
                if need_lower {
                  js_ident = ident.to_lowercase();
                }
                let mut need_replace = true;
                if let Some(block_list) = ignore_component {
                  need_replace = !block_list.iter().map(|c| c.as_str()).any(|x| x == ident);
                }
                if need_replace {
                  let import_es_source =
                    std::string::String::from(replace_expr).replace("{}", js_ident.as_str());
                  specifiers_es.push(EsSpec {
                    source: import_es_source,
                    default_spec: ident,
                  });
                  if !specifiers_rm_es.iter().any(|&c| c == item_index) {
                    specifiers_rm_es.push(item_index);
                  }
                }
              }
            }
            ImportSpecifier::Default(ref _s) => {}
            ImportSpecifier::Namespace(ref _ns) => {}
          }
        }
      }
    }
  }
  let body = &mut module.body;

  let mut index: usize = 0;
  while let Some(i) = specifiers_rm_es.get(index) {
    let rm_index = *i - index;
    body.remove(rm_index);
    index += 1;
  }

  for js_source in specifiers_es {
    let js_source_ref = js_source.source.as_str();
    let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![swc_ecma_ast::ImportSpecifier::Default(
        ImportDefaultSpecifier {
          span: DUMMY_SP,
          local: swc_ecma_ast::Ident {
            span: DUMMY_SP,
            sym: JsWord::from(js_source.default_spec.as_str()),
            optional: false,
          },
        },
      )],
      src: Str {
        span: DUMMY_SP,
        value: JsWord::from(js_source_ref),
        raw: None,
      },
      type_only: false,
      asserts: None,
    }));
    body.insert(0, dec);
  }

  for css_source in specifiers_css {
    let css_source_ref = css_source.as_str();
    let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![],
      src: Str {
        span: DUMMY_SP,
        value: JsWord::from(css_source_ref),
        raw: None,
      },
      type_only: false,
      asserts: None,
    }));
    body.insert(0, dec);
  }
}
