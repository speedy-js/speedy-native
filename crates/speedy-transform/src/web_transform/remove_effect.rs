use std::collections::HashSet;

use swc_common::util::take::Take;
use swc_ecma_ast::{BlockStmt, Expr, Module, ModuleDecl, ModuleExportName, ModuleItem};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::types::TransformConfig;

struct RmUseEffect {
  useEffect_mark: Option<(String, u32)>, // used for remove useEffect()
  react_mark: Option<(String, u32)>,     // used for remove React.useEffect()
}

impl VisitMut for RmUseEffect {
  fn visit_mut_block_stmt(&mut self, n: &mut BlockStmt) {
    let mut rm_idx_set = HashSet::new();
    for (idx, stmt) in n.stmts.iter().enumerate() {
      if let Some(Some(call_expr)) = stmt.as_expr().map(|expr_stmt| expr_stmt.expr.as_call()) {
        call_expr.callee.as_expr().and_then(|callee| {
          match callee.as_ref() {
            Expr::Member(member) => {
              // check React.useEffect call
              member.obj.as_ref().as_ident().and_then(|obj_ident| {
                member.prop.as_ident().and_then(|prop_ident| {
                  if self
                    .react_mark
                    .as_ref()
                    .map(|mark| (mark.0.as_str(), mark.1))
                    .eq(&Some((
                      obj_ident.sym.clone().as_ref(),
                      obj_ident.span.ctxt.as_u32(),
                    )))
                    && prop_ident.clone().as_ref() == "useEffect"
                  {
                    rm_idx_set.insert(idx);
                  };
                  Some(())
                })
              });
            }
            Expr::Ident(ident) => {
              // check useEffect call
              if self
                .useEffect_mark
                .as_ref()
                .map(|mark| (mark.0.as_str(), mark.1))
                .eq(&Some((
                  ident.sym.clone().as_ref(),
                  ident.span.ctxt.as_u32(),
                )))
              {
                rm_idx_set.insert(idx);
              }
            }
            _ => {}
          };
          Some(())
        });
      }
    }
    if !rm_idx_set.is_empty() {
      n.stmts = n
        .stmts
        .take()
        .into_iter()
        .enumerate()
        .filter_map(|(idx, stmt)| {
          if rm_idx_set.contains(&idx) {
            None
          } else {
            Some(stmt)
          }
        })
        .collect();
    }
  }
}

pub fn remove_call(module: &mut Module, config: &TransformConfig) {
  if config.remove_useEffect.is_none() || !config.remove_useEffect.unwrap() {
    return;
  }

  let mut visitor = RmUseEffect {
    react_mark: None,
    useEffect_mark: None,
  };
  for item in &module.body {
    if let ModuleItem::ModuleDecl(ModuleDecl::Import(var)) = item {
      let source = &*var.src.value;
      if source == "react" {
        for specifier in &var.specifiers {
          match specifier {
            swc_ecma_ast::ImportSpecifier::Named(named) => {
              match &named.imported {
                Some(imported) => {
                  let imported_name = match imported {
                    ModuleExportName::Ident(ident) => ident.sym.clone(),
                    ModuleExportName::Str(str) => str.value.clone(),
                  };
                  if imported_name.as_ref() == "useEffect" {
                    // import { useEffect as ??? } from 'react'
                    visitor.useEffect_mark =
                      Some((named.local.sym.to_string(), named.local.span.ctxt.as_u32()));
                  }
                }
                None => {
                  if named.local.sym.clone().as_ref() == "useEffect" {
                    // import { useEffect } from 'react'
                    visitor.useEffect_mark =
                      Some((named.local.sym.to_string(), named.local.span.ctxt.as_u32()));
                  }
                }
              }
            }
            swc_ecma_ast::ImportSpecifier::Default(default) => {
              // import ??? from 'react'
              visitor.react_mark = Some((
                default.local.sym.to_string(),
                default.local.span.ctxt.as_u32(),
              ));
            }
            _ => {}
          }
        }
      }
    }
  }

  module.visit_mut_with(&mut visitor);
}
