use std::collections::HashSet;

use swc_common::util::take::Take;
use swc_ecma_ast::{BlockStmt, Expr, Module};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::types::TransformConfig;

struct RmCallStmt {
  rm_calls: Vec<String>,
}

impl VisitMut for RmCallStmt {
  fn visit_mut_block_stmt(&mut self, n: &mut BlockStmt) {
    let mut rm_idx_set = HashSet::new();
    for (idx, stmt) in n.stmts.iter().enumerate() {
      if let Some(Some(call_expr)) = stmt.as_expr().map(|expr_stmt| expr_stmt.expr.as_call()) {
        call_expr.callee.as_expr().and_then(|callee| {
          let callee_name = match callee.as_ref() {
            Expr::Member(member) => member.prop.as_ident()?.sym.clone(),
            Expr::Ident(ident) => ident.sym.clone(),
            _ => return None,
          };

          if self
            .rm_calls
            .iter()
            .any(|x| x.as_str() == callee_name.as_ref())
          {
            rm_idx_set.insert(idx);
          }
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
  if config.remove_call.is_none() || config.remove_call.as_ref().unwrap().is_empty() {
    return;
  }

  let rm_calls = config.remove_call.as_ref().unwrap().clone();
  let mut visitor = RmCallStmt { rm_calls };
  module.visit_mut_with(&mut visitor);
}
