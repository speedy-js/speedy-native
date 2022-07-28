use std::collections::HashSet;

use crate::types::TransformConfig;
use crate::web_transform::visit::IdentComponent;
use heck::ToKebabCase;
use napi::Env;
use swc::Compiler;
use swc_atoms::JsWord;
use swc_common::{util::take::Take, Mark, DUMMY_SP};
use swc_ecma_ast::{
  Ident, ImportDecl, ImportDefaultSpecifier, ImportNamedSpecifier, ImportSpecifier, ModuleDecl,
  ModuleExportName, ModuleItem, Str,
};
use swc_ecma_transforms::resolver;
use swc_ecma_visit::{VisitMutWith, VisitWith};

use super::clear_mark::ClearMark;

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
  compiler: &Compiler,
) {
  // let s = serde_json::to_string_pretty(&module).expect("failed to serialize");

  compiler.run(|| {
    module.visit_mut_with(&mut resolver(Mark::new(), Mark::new(), true));
  });

  // use visitor to collect all ident reference, and then remove imported component and type that is never referenced
  let mut visitor = IdentComponent {
    ident_set: HashSet::new(),
    type_ident_set: HashSet::new(),
    in_ts_type_ref: false,
  };
  module.body.visit_with(&mut visitor);

  let ident_referenced = |ident: &Ident| -> bool { visitor.ident_set.contains(&ident.to_id()) };
  let type_ident_referenced =
    |ident: &Ident| -> bool { visitor.type_ident_set.contains(&ident.to_id()) };

  if project_config.babel_import.is_none()
    || project_config.babel_import.as_ref().unwrap().is_empty()
  {
    return;
  }
  let mut specifiers_css = vec![];
  let mut specifiers_es = vec![];
  let mut specifiers_rm_es = HashSet::new();

  let config = project_config.babel_import.as_ref().unwrap();

  for (item_index, item) in module.body.iter_mut().enumerate() {
    if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
      let source = &*var.src.value;
      if let Some(child_config) = config.iter().find(|&c| c.from_source == source) {
        let mut rm_specifier = HashSet::new();
        for (specifier_idx, specifier) in var.specifiers.iter().enumerate() {
          match specifier {
            ImportSpecifier::Named(ref s) => {
              let imported = s.imported.as_ref().map(|imported| match imported {
                ModuleExportName::Ident(ident) => ident.sym.to_string(),
                ModuleExportName::Str(str) => str.value.to_string(),
              });
              // 当 imported 不为 none 时, local.sym 是引入组件的 as 别名
              let as_name: Option<String> = imported.is_some().then(|| s.local.sym.to_string());
              // 当 imported 不为 none 时, 实际引入的组件命名为 imported, 否则为 s.local.sym
              let ident: String = imported.unwrap_or_else(|| s.local.sym.to_string());
              if ident_referenced(&s.local) {
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
                    rm_specifier.insert(specifier_idx);
                  }
                }
              } else if type_ident_referenced(&s.local) {
                // type referenced
                continue;
              } else {
                // not referenced, should tree shaking
                rm_specifier.insert(specifier_idx);
              }
            }
            ImportSpecifier::Default(ref _s) => {}
            ImportSpecifier::Namespace(ref _ns) => {}
          }
        }
        if rm_specifier.len() == var.specifiers.len() {
          // all specifier remove, just remove whole stmt
          specifiers_rm_es.insert(item_index);
        } else {
          // only remove some specifier
          var.specifiers = var
            .specifiers
            .take()
            .into_iter()
            .enumerate()
            .filter_map(|(idx, spec)| (!rm_specifier.contains(&idx)).then(|| spec))
            .collect();
        }
      }
    }
  }

  module.body = module
    .body
    .take()
    .into_iter()
    .enumerate()
    .filter_map(|(idx, stmt)| (!specifiers_rm_es.contains(&idx)).then(|| stmt))
    .collect();

  let body = &mut module.body;

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

  module.visit_mut_with(&mut ClearMark);
}
