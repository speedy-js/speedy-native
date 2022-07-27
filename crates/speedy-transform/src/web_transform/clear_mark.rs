use swc_common::SyntaxContext;
use {swc_ecma_ast::Ident, swc_ecma_visit::VisitMut};

#[derive(Clone, Copy)]
pub struct ClearMark;

impl VisitMut for ClearMark {
  fn visit_mut_ident(&mut self, ident: &mut Ident) {
    ident.span.ctxt = SyntaxContext::empty();
  }
}
