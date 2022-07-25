use swc_ecma_ast::{BlockStmt, Expr, Module};
use swc_ecma_visit::{VisitMut, VisitMutWith};

use crate::types::TransformConfig;

struct RmCallStmt {
  rm_calls: Vec<String>,
}

impl VisitMut for RmCallStmt {
  fn visit_mut_block_stmt(&mut self, n: &mut BlockStmt) {
    let mut rm_idx = vec![];
    for (idx, stmt) in n.stmts.iter().enumerate() {
      if let Some(Some(call_expr)) = stmt.as_expr().map(|expr_stmt| expr_stmt.expr.as_call()) {
        if let Some(callee) = call_expr.callee.as_expr() {
          let callee_name = match callee.as_ref() {
            Expr::Member(member) => member
              .prop
              .as_ident()
              .map_or("".to_string(), |ident| ident.sym.to_string()),
            Expr::Ident(ident) => ident.sym.to_string(),
            _ => "".to_string(),
          };

          if self.rm_calls.iter().any(|x| x == &callee_name) {
            rm_idx.push(idx);
          }
        }
      }
    }
    let mut index: usize = 0;
    while let Some(i) = rm_idx.get(index) {
      let rm_index = *i - index;
      n.stmts.remove(rm_index);
      index += 1;
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
