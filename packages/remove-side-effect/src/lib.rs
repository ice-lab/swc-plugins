use swc_core::ecma::{
    ast::*,
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

#[derive(Default)]
pub struct TransformVisitor {
    react_imports: Vec<String>,
    imported_hooks: Vec<String>,
    scope_stack: Vec<Vec<String>>,
}

impl TransformVisitor {
    pub fn new() -> Self {
        Self {
            react_imports: Vec::new(),
            imported_hooks: Vec::new(),
            scope_stack: vec![Vec::new()],
        }
    }

    fn target_hooks() -> Vec<&'static str> {
        vec!["useEffect", "useLayoutEffect"]
    }

    fn is_target_hook(name: &str) -> bool {
        Self::target_hooks().contains(&name)
    }

    fn enter_scope(&mut self) {
        self.scope_stack.push(Vec::new());
    }

    fn exit_scope(&mut self) {
        self.scope_stack.pop();
    }

    fn add_to_current_scope(&mut self, name: String) {
        if let Some(scope) = self.scope_stack.last_mut() {
            scope.push(name);
        }
    }

    fn is_local_variable(&self, name: &str) -> bool {
        for scope in self.scope_stack.iter().rev() {
            if scope.contains(&name.to_string()) {
                return true;
            }
        }
        false
    }

    fn is_react_hook(&self, expr: &Expr) -> bool {
        match expr {
            Expr::Ident(ident) => {
                let name = ident.sym.to_string();
                if Self::is_target_hook(&name) {
                    if self.is_local_variable(&name) {
                        return false;
                    }
                    return self.imported_hooks.contains(&name);
                }
            }
            Expr::Member(member) => {
                if let Expr::Ident(obj) = &*member.obj {
                    if let Some(prop) = &member.prop.as_ident() {
                        return self.react_imports.contains(&obj.sym.to_string())
                            && Self::is_target_hook(&prop.sym.to_string());
                    }
                }
            }
            _ => {}
        }
        false
    }
}

impl VisitMut for TransformVisitor {
    // when into any function, find same effect var exist or not.
    fn visit_mut_function(&mut self, func: &mut Function) {
        self.enter_scope();
        for param in &func.params {
            if let Pat::Ident(ident) = &param.pat {
                self.add_to_current_scope(ident.id.sym.to_string());
            }
        }
        func.visit_mut_children_with(self);
        self.exit_scope();
    }

    fn visit_mut_arrow_expr(&mut self, arrow: &mut ArrowExpr) {
        self.enter_scope();
        for param in &arrow.params {
            if let Pat::Ident(ident) = param {
                self.add_to_current_scope(ident.sym.to_string());
            }
        }
        arrow.visit_mut_children_with(self);
        self.exit_scope();
    }

    fn visit_mut_var_decl(&mut self, var_decl: &mut VarDecl) {
        for decl in &var_decl.decls {
            if let Pat::Ident(ident) = &decl.name {
                self.add_to_current_scope(ident.id.sym.to_string());
            }
        }
        var_decl.visit_mut_children_with(self);
    }

    fn visit_mut_module(&mut self, module: &mut Module) {
        for item in &module.body {
            if let ModuleItem::ModuleDecl(ModuleDecl::Import(import)) = item {
                if import.src.value.to_string() == "react" {
                    for spec in &import.specifiers {
                        match spec {
                            ImportSpecifier::Named(named) => {
                                let name = named.local.sym.to_string();
                                if Self::is_target_hook(&name) {
                                    self.imported_hooks.push(name);
                                }
                            }
                            ImportSpecifier::Default(default_import) => {
                                self.react_imports
                                    .push(default_import.local.sym.to_string());
                            }
                            ImportSpecifier::Namespace(namespace) => {
                                self.react_imports.push(namespace.local.sym.to_string());
                            }
                        }
                    }
                }
            }
        }

        module.visit_mut_children_with(self);
    }

    fn visit_mut_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        for stmt in stmts.iter_mut() {
            stmt.visit_mut_children_with(self);
        }

        stmts.retain(|stmt| {
            if let Stmt::Expr(expr_stmt) = stmt {
                if let Expr::Call(call_expr) = &*expr_stmt.expr {
                    if let Callee::Expr(callee) = &call_expr.callee {
                        return !self.is_react_hook(callee);
                    }
                }
            }
            true
        });
    }

    fn visit_mut_block_stmt(&mut self, block: &mut BlockStmt) {
        self.enter_scope();
        block.visit_mut_children_with(self);
        self.exit_scope();
    }

    // try-catch
    fn visit_mut_try_stmt(&mut self, try_stmt: &mut TryStmt) {
        self.enter_scope();
        try_stmt.block.visit_mut_children_with(self);
        self.exit_scope();

        // catch
        if let Some(catch) = &mut try_stmt.handler {
            self.enter_scope();
            // 添加 catch 参数到作用域
            if let Some(Pat::Ident(ident)) = &catch.param {
                self.add_to_current_scope(ident.sym.to_string());
            }
            catch.body.visit_mut_children_with(self);
            self.exit_scope();
        }

        // finally
        if let Some(finally) = &mut try_stmt.finalizer {
            self.enter_scope();
            finally.visit_mut_children_with(self);
            self.exit_scope();
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor::new()))
}
