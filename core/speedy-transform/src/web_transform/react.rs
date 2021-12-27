use crate::types::TransformConfig;
use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{ImportDecl, ImportDefaultSpecifier, ModuleDecl, ModuleItem, Str};

pub fn transform_perfixreact(
  module: &mut swc_ecma_ast::Module,
  project_config: &TransformConfig,
  origin_code: &str,
) {
  if project_config.reat_runtime.unwrap_or(false) {
    if origin_code.contains("import React from \"react\"")
      || origin_code.contains("import React from 'react'")
    {
      return;
    }
    let body = &mut module.body;

    let dec = ModuleItem::ModuleDecl(ModuleDecl::Import(ImportDecl {
      span: DUMMY_SP,
      specifiers: vec![swc_ecma_ast::ImportSpecifier::Default(
        ImportDefaultSpecifier {
          span: DUMMY_SP,
          local: swc_ecma_ast::Ident {
            span: DUMMY_SP,
            sym: JsWord::from("React"),
            optional: false,
          },
        },
      )],
      src: Str {
        span: DUMMY_SP,
        value: JsWord::from("react"),
        has_escape: false,
        kind: Default::default(),
      },
      type_only: false,
      asserts: None,
    }));
    body.insert(0, dec);
  }
}
