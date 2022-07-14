use crate::types::TransformConfig;
use crate::web_transform::visit::IdentComponent;
use heck::ToKebabCase;
use napi::Env;
use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  Ident, ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier, ModuleDecl,
  ModuleExportName, ModuleItem, Str,
};
use swc_ecma_visit::VisitWith;

struct EsSpec {
  source: String,
  default_spec: String,
  as_name: Option<String>,
  use_default_import: bool,
}

pub fn transform_style(
  env: Env,
  module: &mut swc_ecma_ast::Module,
  project_config: &TransformConfig,
) {
  // let s = serde_json::to_string_pretty(&module).expect("failed to serialize");

  let mut visitor = IdentComponent {
    component_name_jsx_ident: vec![],
    ident_list: vec![],
  };
  module.body.visit_with(&mut visitor);

  let match_ident = |idnet: &Ident| -> bool {
    let name = idnet.to_string().replace("#0", "");
    let mark = idnet.span.ctxt.as_u32();
    let item = (name, mark);
    visitor.component_name_jsx_ident.contains(&item) || visitor.ident_list.contains(&item)
  };

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
              let imported = match &s.imported {
                Some(imported) => match &imported {
                  ModuleExportName::Ident(ident) => Some(ident.sym.to_string()),
                  ModuleExportName::Str(ident) => Some(ident.value.to_string()),
                },
                None => None,
              };
              // 当 imported 不为 none 时, local.sym 是引入组件的 as 别名
              let as_name: Option<String> = if imported.is_some() {
                Some(s.local.sym.to_string())
              } else {
                None
              };
              // 当 imported 不为 none 时, 实际引入的组件命名为 imported, 否则为 s.local.sym
              let ident: String = imported.unwrap_or(s.local.sym.to_string());

              if match_ident(&s.local) {
                // 替换对应的 css
                if let Some(ref css) = child_config.replace_css {
                  let ignore_component = &css.ignore_style_component;
                  let need_lower = css.lower.unwrap_or(false);
                  let camel2dash = css.camel2_dash_component_name.unwrap_or(false);
                  let mut css_ident = ident.clone();
                  if camel2dash {
                    css_ident = css_ident.to_kebab_case();
                  }
                  if need_lower {
                    css_ident = css_ident.to_lowercase()
                  };
                  let mut need_replace = true;
                  if let Some(block_list) = ignore_component {
                    need_replace = !block_list.iter().any(|x| x == &ident);
                  }
                  if need_replace {
                    let import_css_source = css
                      .replace_expr
                      .call(None, &[env.create_string(css_ident.as_str()).unwrap()])
                      .unwrap()
                      .coerce_to_string()
                      .unwrap()
                      .into_utf8()
                      .unwrap()
                      .as_str()
                      .unwrap()
                      .to_string();
                    specifiers_css.push(import_css_source);
                  }
                }
                // 替换对应 spec 的js
                if let Some(ref js_config) = child_config.replace_js {
                  let ignore_component = &js_config.ignore_es_component;
                  let need_lower = js_config.lower.unwrap_or(false);
                  let camel2dash = js_config.camel2_dash_component_name.unwrap_or(false);
                  let use_default_import = js_config.transform_to_default_import.unwrap_or(true);
                  let mut js_ident = ident.clone();
                  if camel2dash {
                    js_ident = js_ident.to_kebab_case();
                  }
                  if need_lower {
                    js_ident = js_ident.to_lowercase();
                  }
                  let mut need_replace = true;
                  if let Some(block_list) = ignore_component {
                    need_replace = !block_list.iter().any(|x| x == &ident);
                  }
                  if need_replace {
                    let import_es_source = js_config
                      .replace_expr
                      .call(None, &[env.create_string(js_ident.as_str()).unwrap()])
                      .unwrap()
                      .coerce_to_string()
                      .unwrap()
                      .into_utf8()
                      .unwrap()
                      .as_str()
                      .unwrap()
                      .to_string();
                    specifiers_es.push(EsSpec {
                      source: import_es_source,
                      default_spec: ident,
                      as_name,
                      use_default_import,
                    });
                    if !specifiers_rm_es.iter().any(|&c| c == item_index) {
                      specifiers_rm_es.push(item_index);
                    }
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
      specifiers: if js_source.use_default_import {
        vec![swc_ecma_ast::ImportSpecifier::Default(
          ImportDefaultSpecifier {
            span: DUMMY_SP,
            local: swc_ecma_ast::Ident {
              span: DUMMY_SP,
              sym: JsWord::from(js_source.as_name.unwrap_or(js_source.default_spec).as_str()),
              optional: false,
            },
          },
        )]
      } else {
        vec![swc_ecma_ast::ImportSpecifier::Named(ImportNamedSpecifier {
          span: DUMMY_SP,
          imported: if js_source.as_name.is_some() {
            Some(swc_ecma_ast::ModuleExportName::Ident(swc_ecma_ast::Ident {
              span: DUMMY_SP,
              sym: JsWord::from(js_source.default_spec.as_str()),
              optional: false,
            }))
          } else {
            None
          },
          local: swc_ecma_ast::Ident {
            span: DUMMY_SP,
            sym: JsWord::from(js_source.as_name.unwrap_or(js_source.default_spec).as_str()),
            optional: false,
          },
          is_type_only: false,
        })]
      },
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
