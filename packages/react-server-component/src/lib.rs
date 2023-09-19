use serde::Deserialize;
use swc_core::{
  common::{
    comments::{Comment, CommentKind, Comments},
    FileName,
    Span,DUMMY_SP,
    errors::HANDLER
  },
  ecma::{
      ast::*,
      atoms::{JsWord, js_word},
      visit::{Fold, FoldWith, VisitMut, as_folder, noop_visit_mut_type, VisitMutWith},
      utils::{prepend_stmts, quote_ident, quote_str, ExprFactory}
  },
  plugin::{plugin_transform, proxies::{TransformPluginProgramMetadata, PluginCommentsProxy}, metadata::TransformPluginMetadataContextKind},
};

struct ModuleImports {
  source: (JsWord, Span),
  specifiers: Vec<(JsWord, Span)>,
}

pub fn react_server_component<C>(file_name: FileName, is_server: bool, comments: C) -> impl Fold + VisitMut
where C: Comments,
{
  as_folder(ReactServerComponent {
    comments,
    filepath: file_name.to_string(),
    is_server,
    export_names: vec![],
    invalid_server_imports: vec![
      JsWord::from("client-only"),
    ],
    invalid_client_imports: vec![
      JsWord::from("server-only"),
    ],
    invalid_server_react_apis: vec![
      JsWord::from("Component"),
      JsWord::from("createContext"),
      JsWord::from("createFactory"),
      JsWord::from("PureComponent"),
      JsWord::from("useDeferredValue"),
      JsWord::from("useEffect"),
      JsWord::from("useImperativeHandle"),
      JsWord::from("useInsertionEffect"),
      JsWord::from("useLayoutEffect"),
      JsWord::from("useReducer"),
      JsWord::from("useRef"),
      JsWord::from("useState"),
      JsWord::from("useSyncExternalStore"),
      JsWord::from("useTransition"),
    ],
    // TODO: add more apis to this list which are not supported in server components.
    invalid_server_ice_imports: vec![
      JsWord::from("useData"),
    ],
  })
}

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Config {
  pub is_server: bool
}

struct ReactServerComponent<C: Comments> {
  filepath: String,
  is_server: bool,
  comments: C,
  export_names: Vec<String>,
  invalid_server_imports: Vec<JsWord>,
  invalid_client_imports: Vec<JsWord>,
  invalid_server_react_apis: Vec<JsWord>,
  invalid_server_ice_imports: Vec<JsWord>,
}
impl<C: Comments> ReactServerComponent<C>  {
  fn create_module_proxy(&self, module: &mut Module) {
    // Clear all statements and module decalarations.
    module.body.clear();
    let file_path = quote_str!(&*self.filepath);

    prepend_stmts(
      &mut module.body,
      vec![
        // const { createClientModuleProxy } = require(\'react-server-dom-webpack/server.node\');
        ModuleItem::Stmt(Stmt::Decl(Decl::Var(Box::new(VarDecl {
          span: DUMMY_SP,
          kind: VarDeclKind::Const,
          decls: vec![
            VarDeclarator {
              span: DUMMY_SP,
              name: Pat::Object(ObjectPat {
                span: DUMMY_SP,
                props: vec![ObjectPatProp::Assign(AssignPatProp {
                  span: DUMMY_SP,
                  key: quote_ident!("createClientModuleProxy"),
                  value: None,
                })],
                optional: false,
                type_ann: None,
              }),
              init: Some(Box::new(Expr::Call(CallExpr {
                span: DUMMY_SP,
                callee: quote_ident!("require").as_callee(),
                args: vec![quote_str!("react-server-dom-webpack/server.node").as_arg()],
                type_args: Default::default(),
              }))),
              definite: false,
            },
          ],
          declare: false,
        })))),
        // module.exports = createClientModuleProxy(moduleId),
        ModuleItem::Stmt(Stmt::Expr(ExprStmt {
          span: DUMMY_SP,
          expr: Box::new(Expr::Assign(AssignExpr {
            span: DUMMY_SP,
            left: PatOrExpr::Expr(Box::new(Expr::Member(MemberExpr {
              span: DUMMY_SP,
              obj: Box::new(Expr::Ident(quote_ident!("module"))),
              prop: MemberProp::Ident(quote_ident!("exports")),
            }))),
            op: op!("="),
            right: Box::new(Expr::Call(CallExpr {
              span: DUMMY_SP,
              callee: quote_ident!("createClientModuleProxy").as_callee(),
              args: vec![file_path.as_arg()],
              type_args: Default::default(),
            })),
          })),
        })),
      ].into_iter(),
    );
    self.prepend_comment_node(module);
  }
  
  fn collect_top_level_directives_and_imports(&mut self, module: &mut Module) -> (bool, bool, Vec<ModuleImports>) {
    let mut imports: Vec<ModuleImports> = vec![];
    let mut finished_directives = false;
    let mut is_client_entry = false;
    let mut is_action_file = false;

    fn panic_both_directives(span: Span) {
      // Error handle for both directives in the same file.
      HANDLER.with(|handler| {
        handler
          .struct_span_err(
            span,
            "Cannot use both `use client` and `use server` in the same file.",
          )
          .emit()
      })
    }
    
    let _ = &module.body.retain(|item| {
      match item {
        ModuleItem::Stmt(stmt) => {
          if !stmt.is_expr() {
            finished_directives = true;
          }

          match stmt.as_expr() {
            Some(expr_stmt) => {
              match &*expr_stmt.expr {
                Expr::Lit(Lit::Str(Str { value, ..})) => {
                  if &**value == "use client" {
                    if !finished_directives {
                      is_client_entry = true;

                      if is_action_file {
                        panic_both_directives(expr_stmt.span)
                      }
                    } else {
                      HANDLER.with(|handler| {
                        handler
                          .struct_span_err(
                            expr_stmt.span,
                            "The \"use client\" directive must be placed before other expressions. Move it to the top of the file to resolve this issue.",
                          ).emit()
                      })
                    }
                    // Remove the directive.
                    return false;
                  } else if &**value == "use server" && !finished_directives {
                    is_action_file = true;

                    if is_client_entry {
                      panic_both_directives(expr_stmt.span)
                    }
                  }
                }
                // Case `("use client;")`.
                Expr::Paren(ParenExpr { expr, .. }) => {
                  finished_directives = true;
                  if let Expr::Lit(Lit::Str(Str { value, .. })) = &**expr {
                      if &**value == "use client" {
                          HANDLER.with(|handler| {
                              handler
                                  .struct_span_err(
                                      expr_stmt.span,
                                      "\"use client\" must be a directive, and placed before other expressions. Remove the parentheses and move it to the top of the file to resolve this issue.",
                                  )
                                  .emit()
                          })
                      }
                  }
                }
                _ => {
                  // Other expression types.
                  finished_directives = true;
                }
              }
            }
            None => {
              // Not an expression.
              finished_directives = true;
            }
          }
        }
        // Collect import specifiers.
        ModuleItem::ModuleDecl(ModuleDecl::Import(import)) => {
          let source = import.src.value.clone();
          let specifiers = import.specifiers.iter().map(|specifier| match specifier {
            ImportSpecifier::Named(named) => match &named.imported {
              Some(imported) => match &imported {
                ModuleExportName::Ident(i) => (i.to_id().0, i.span),
                ModuleExportName::Str(s) => (s.value.clone(), s.span),
              },
              None => (named.local.to_id().0, named.local.span),
            },
            ImportSpecifier::Default(d) => (js_word!(""), d.span),
            ImportSpecifier::Namespace(n) => ("*".into(), n.span),
          })
          .collect();
          imports.push(ModuleImports { source: (source, import.span), specifiers });
          finished_directives = true;
        }
        // Collect all export names.
        ModuleItem::ModuleDecl(ModuleDecl::ExportNamed(e)) => {
          for specifier in &e.specifiers {
            self.export_names.push(match specifier {
              ExportSpecifier::Default(_) => "default".to_string(),
              ExportSpecifier::Namespace(_) => "*".to_string(),
              ExportSpecifier::Named(named) => match &named.exported {
                Some(exported) => match &exported {
                  ModuleExportName::Ident(i) => i.sym.to_string(),
                  ModuleExportName::Str(s) => s.value.to_string(),
                },
                _ => match &named.orig {
                  ModuleExportName::Ident(i) => i.sym.to_string(),
                  ModuleExportName::Str(s) => s.value.to_string(),
                },
              },
            })
          }
          finished_directives = true;
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(ExportDecl { decl, ..})) => {
          match decl {
            Decl::Class(ClassDecl { ident, .. }) => {
              self.export_names.push(ident.sym.to_string());
            }
            Decl::Fn(FnDecl { ident, .. }) => {
              self.export_names.push(ident.sym.to_string());
            }
            Decl::Var(var) => {
              for decl in &var.decls {
                if let Pat::Ident(ident) = &decl.name {
                  self.export_names.push(ident.sym.to_string());
                }
              }
            }
            _ => {}
          }
          finished_directives = true;
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultDecl(_)) => {
          self.export_names.push("default".to_string());
          finished_directives = true;
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportDefaultExpr(_)) => {
          self.export_names.push("default".to_string());
          finished_directives = true;
        }
        ModuleItem::ModuleDecl(ModuleDecl::ExportAll(_)) => {
          self.export_names.push("*".to_string());
        }
        _ => {
          finished_directives = true;
        }
      }
      true
    }); 
    (is_client_entry, is_action_file, imports)
  }

  fn assert_server_import(&self, imports: &Vec<ModuleImports>) {
    for import in imports {
      let source = import.source.0.clone();
      if self.invalid_server_imports.contains(&source) {
        HANDLER.with(|handler| {
          handler
            .struct_span_err(
              import.source.1,
              format!("Cannot import \"{}\" in a server component.", source).as_str(),
            )
            .emit()
        })
      }
      if source == *"react" {
        for specifier in &import.specifiers {
          if self.invalid_server_react_apis.contains(&specifier.0) {
            HANDLER.with(|handler| {
              handler
                .struct_span_err(
                  specifier.1,
                  format!("Cannot use react API: \"{}\" in a server component.", specifier.0).as_str(),
                )
                .emit()
            })
          }
        }
      }
      if source == *"ice" {
        for specifier in &import.specifiers {
          if self.invalid_server_ice_imports.contains(&specifier.0) {
            HANDLER.with(|handler| {
              handler
                .struct_span_err(
                  specifier.1,
                  format!("Cannot use ice API: \"{}\" in a server component.", specifier.0).as_str(),
                )
                .emit()
            })
          }
        }
      }
    }
  }

  fn assert_client_import(&self, imports: &Vec<ModuleImports>) {
    for import in imports {
      let source = import.source.0.clone();
      if self.invalid_client_imports.contains(&source) {
        HANDLER.with(|handler| {
          handler
            .struct_span_err(
              import.source.1,
              format!("Cannot import \"{}\" in a client component.", source).as_str(),
            )
            .emit()
        })
      }
    }
  }

  fn prepend_comment_node(&self, module: &Module) {
    // Prepend a special comment at the top of file, so that we can identify client boundary in webpack plugin
    // just by reading the firist line of file.
    self.comments.add_leading(
      module.span.lo,
      Comment {
        kind: CommentKind::Block,
        span: DUMMY_SP,
        text: format!("__ice_internal_client_entry_do_not_use__ {}", self.export_names.join(",")).into(),
      },
    );
  }
}

impl <C: Comments> VisitMut for ReactServerComponent<C> {
  noop_visit_mut_type!();

  fn visit_mut_module(&mut self, module: &mut Module) {
    let (is_client_entry, is_action_file, imports) = self.collect_top_level_directives_and_imports(module);
    tracing::debug!("is_client_entry: {}, is_action_file: {}", is_client_entry, is_action_file);
    if self.is_server {
      if !is_client_entry {
        self.assert_server_import(&imports);
      } else {
        // Proxy client module.
        self.create_module_proxy(module);
        return;
      }
    } else {
      if !is_action_file {
        self.assert_client_import(&imports);
      }
      if is_client_entry {
        self.prepend_comment_node(module);
      }
    }
    module.visit_mut_children_with(self)
  }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
  let config = serde_json::from_str::<Config>(
    &_metadata
      .get_transform_plugin_config()
      .expect("failed to get plugin config for react-server-component"),
  )
  .expect("invalid config for react-server-component");
  let file_name = match _metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
    Some(s) => FileName::Real(s.into()),
    None => FileName::Anon,
  };
  program.fold_with(&mut react_server_component(file_name, config.is_server, PluginCommentsProxy))
}