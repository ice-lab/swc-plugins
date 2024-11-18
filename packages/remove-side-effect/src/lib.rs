use swc_core::ecma::{
    ast::*,
    visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    fn visit_mut_expr(&mut self, expr: &mut Expr) {
        // 首先递归访问子节点
        expr.visit_mut_children_with(self);

        // 检查是否是函数调用
        if let Expr::Call(call_expr) = expr {
            // 检查被调用的是否是 useEffect
            if let Callee::Expr(callee) = &call_expr.callee {
                if let Expr::Ident(ident) = &**callee {
                    if ident.sym.to_string() == "useEffect" {
                        // 将 useEffect 调用替换为空语句
                        *expr = Expr::Lit(Lit::Null(Null {
                            span: call_expr.span,
                        }));
                    }
                }
            }
        }
    }

    fn visit_mut_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_mut_children_with(self);

        // 如果语句只包含一个被我们替换为 null 的表达式，则移除整个语句
        if let Stmt::Expr(expr_stmt) = stmt {
            if let Expr::Lit(Lit::Null(_)) = &*expr_stmt.expr {
                *stmt = Stmt::Empty(EmptyStmt {
                    span: expr_stmt.span,
                });
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}
