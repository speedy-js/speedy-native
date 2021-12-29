use std::collections::HashMap;
use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ExportDecl, ImportDecl, ImportDefaultSpecifier, ImportSpecifier, Module, ModuleDecl, ModuleItem,
  Str, VisitMut,
};
use swc_ecma_visit::{noop_visit_mut_type, VisitMut, VisitMutWith};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ImportDesc {
  pub source: JsWord,
  // name in importer
  pub name: JsWord,
  // orignal defined name
  pub local_name: JsWord,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ExportDesc {
  pub identifier: Option<JsWord>,
  pub local_name: JsWord,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ReExportDesc {
  // name in importee
  pub name: JsWord,
  // locally defined name
  pub local_name: JsWord,
  pub source: JsWord,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct DynImportDesc {
  pub argument: JsWord,
  pub id: Option<JsWord>,
}

struct SystemJs {
  imports: HashMap<JsWord, ImportDesc>,
  exports: HashMap<JsWord, ExportDecl>,
}

impl SystemJs {
  pub fn add_import(&mut self, node: &mut ModuleDecl) {}

  pub fn add_export(node: &mut ModuleDecl) {}
}

impl VisitMut for SystemJs {
  noop_visit_mut_type!();

  fn visit_mut_module_decl(&mut self, node: &mut ModuleDecl) {
    node.visit_mut_children_with(self);
  }
}
