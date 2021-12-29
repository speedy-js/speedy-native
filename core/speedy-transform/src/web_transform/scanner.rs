use std::collections::HashMap;
use swc_atoms::JsWord;
use swc_common::DUMMY_SP;
use swc_ecma_ast::{
  ExportDecl, ExportSpecifier, ImportDecl, ImportDefaultSpecifier, ImportSpecifier, Module,
  ModuleDecl, ModuleItem, Str, VisitMut,
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

struct Scanner {
  imports: HashMap<JsWord, ImportDesc>,
  exports: HashMap<JsWord, ExportDesc>,
}

impl Scanner {
  pub fn new() -> Self {
    Self {
      imports: Default::default(),
      exports: Default::default(),
    }
  }
  pub fn add_import(&mut self, module_decl: &mut ModuleDecl) {
    match module_decl {
      ModuleDecl::Import(import_decl) => {
        if import_decl.specifiers.len() > 0 {
          import_decl
            .specifiers
            .iter()
            .for_each(|specifier| match specifier {
              ImportSpecifier::Named(s) => {
                self.imports.insert(
                  s.local.sym.clone(),
                  ImportDesc {
                    source: import_decl.src.value.clone(),
                    name: s.imported.unwrap().sym.clone(),
                    local_name: s.local.sym.clone(),
                  },
                );
              }
              ImportSpecifier::Default(s) => {
                self.imports.insert(
                  s.local.sym.clone(),
                  ImportDesc {
                    source: import_decl.src.value.clone(),
                    name: "default".to_owned(),
                    local_name: s.local.sym.clone(),
                  },
                );
              }
              ImportSpecifier::Namespace(s) => {
                self.imports.insert(
                  s.local.sym.clone(),
                  ImportDesc {
                    source: import_decl.src.value.clone(),
                    name: "*".to_owned(),
                    local_name: s.local.sym.clone(),
                  },
                );
              }
            })
        }
      }
      _ => {}
    }
  }
  pub fn add_export(&mut self, module_decl: &mut ModuleDecl) {
    match module_decl {
      ModuleDecl::ExportNamed(e) => e.specifiers.iter().for_each(|s| match s {
        ExportSpecifier::Namespace(s) => {
          self.exports.insert(
            s.name.sym.clone(),
            ExportDesc {
              identifier: Some(s.name.sym.clone()),
              local_name: "asdf",
            },
          );
        }
        ExportSpecifier::Default(s) => {
          todo!()
        }
        ExportSpecifier::Named(s) => {
          todo!()
        }
      }),
      _ => {}
    }
  }
}

impl VisitMut for Scanner {
  noop_visit_mut_type!();

  fn visit_mut_module_decl(&mut self, node: &mut ModuleDecl) {
    node.visit_mut_children_with(self);
  }
}
