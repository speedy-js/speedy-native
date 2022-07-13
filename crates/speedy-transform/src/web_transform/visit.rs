use swc_ecma_ast::{Ident, ImportDecl, JSXElement, JSXElementName, TsEntityName, TsTypeRef};
use swc_ecma_visit::Visit;
use swc_ecma_visit::VisitWith;

#[derive(Default)]
pub struct IdentComponent {
  pub component_name_jsx_ident: Vec<(String, u32)>,
  pub ident_list: Vec<(String, u32)>,
  pub ts_type_ident_list: Vec<(String, u32)>,
}

///
/// 处理 babel_import 自动 treeshaking 的问题
/// 增加 判断 jsx 所有引用的关系
///
impl Visit for IdentComponent {
  // need to skip import decl
  fn visit_import_decl(&mut self, _: &ImportDecl) {}

  fn visit_jsx_element(&mut self, jsx: &JSXElement) {
    let mut compent_name = match &jsx.opening.name {
      JSXElementName::Ident(ident) => (ident.to_string(), ident.span.ctxt.as_u32()),
      JSXElementName::JSXMemberExpr(member) => {
        (member.prop.to_string(), member.prop.span.ctxt.as_u32())
      }
      JSXElementName::JSXNamespacedName(space) => {
        (space.name.to_string(), space.name.span.ctxt.as_u32())
      }
    };
    compent_name.0 = compent_name.0.replace("#0", "");
    self.component_name_jsx_ident.push(compent_name);
    jsx.children.visit_with(self);
  }

  fn visit_ident(&mut self, ident: &Ident) {
    self
      .ident_list
      .push((ident.sym.to_string(), ident.span.ctxt.as_u32()));
  }

  fn visit_ts_type_ref(&mut self, ts_type: &TsTypeRef) {
    if let TsEntityName::Ident(ident) = &ts_type.type_name {
      self
        .ts_type_ident_list
        .push((ident.sym.to_string(), ident.span.ctxt.as_u32()));
    }
  }
}
