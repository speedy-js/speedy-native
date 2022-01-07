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

pub fn transformstyle(module: &mut swc_ecma_ast::Module, project_config: &TransformConfig) {
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
      if let Some(childconfig) = config.iter().find(|&c| c.from_source == source) {
        for specifier in &var.specifiers {
          match specifier {
            ImportSpecifier::Named(ref s) => {
              let ident = format!("{}", s.local.sym);
              // 替换对应的 css
              if childconfig.replace_css.is_some() {
                let css = childconfig.replace_css.as_ref().unwrap();
                let replace_expr = css.replace_expr.as_str();
                let ignore_component = &css.ignore_style_component;
                let needlower = css.lower.unwrap_or(false);
                let mut css_ident = ident.clone();
                if needlower {
                  css_ident = ident.to_lowercase();
                }
                let mut need_replace = true;
                if (*ignore_component).is_some() {
                  let blocklist = (*ignore_component).as_ref().unwrap();
                  blocklist.iter().map(|c| c.as_str()).for_each(|x| {
                    if x == ident {
                      need_replace = false;
                    }
                  });
                }
                if need_replace {
                  let import_css_source =
                    std::string::String::from(replace_expr).replace("{}", css_ident.as_str());
                  specifiers_css.push(import_css_source);
                }
              }
              // 替换对应 spec 的js
              if childconfig.replace_js.is_some() {
                let js = childconfig.replace_js.as_ref().unwrap();
                let replace_expr = js.replace_expr.as_str();
                let ignore_component = &js.ignore_es_component;
                let needlower = js.lower.unwrap_or(false);
                let mut js_ident = ident.clone();
                if needlower {
                  js_ident = ident.to_lowercase();
                }
                let mut need_replace = true;
                if (*ignore_component).is_some() {
                  let blocklist = (*ignore_component).as_ref().unwrap();
                  blocklist.iter().map(|c| c.as_str()).for_each(|x| {
                    if x == ident {
                      need_replace = false;
                    }
                  });
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
  loop {
    if specifiers_rm_es.get(index).is_none() {
      break;
    } else {
      let rm_index = specifiers_rm_es.get(index).unwrap() - index;
      body.remove(rm_index);
    }
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
        has_escape: false,
        kind: Default::default(),
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
        has_escape: false,
        kind: Default::default(),
      },
      type_only: false,
      asserts: None,
    }));
    body.insert(0, dec);
  }
}
