use im::HashMap;
use int_hash::IntBuildHasher;
use syntax::ast::*;
use syntax::Symbol;

use automaton::state::StateId;
use automaton::{nfa, Automaton};
use variance::Pos;
use {polar, var, Var};

#[derive(Default)]
struct Context {
    var: var::Context,
    auto: Automaton,
    env: Vec<StateId>,
    names: HashMap<Symbol, Var, IntBuildHasher>,
}

fn type_check<T: Check>(ast: T) -> nfa::Scheme {
    let pcx = polar::Context::new();
    let mut cx = Context::default();
    let ty = ast.check(&pcx, &mut cx);
    let expr = cx.auto.build(ty);
    nfa::Scheme::from_parts(cx.auto, cx.env, expr)
}

trait Check {
    fn check<'c>(&self, pcx: &'c polar::Context<'c>, cx: &mut Context) -> polar::Ty<'c, Pos>;
}

impl Check for Stmt {
    fn check<'c>(&self, pcx: &'c polar::Context<'c>, cx: &mut Context) -> polar::Ty<'c, Pos> {
        match self {
            Stmt::Expr(expr) => {
                expr.check(pcx, cx);
                polar::TyKind::Zero::<Pos>.intern(pcx)
            }
            Stmt::Bind(bind) => bind.check(pcx, cx),
        }
    }
}

impl Check for Bind {
    fn check<'c>(&self, pcx: &'c polar::Context<'c>, cx: &mut Context) -> polar::Ty<'c, Pos> {
        let var = cx.var.unbound();
        cx.names.insert(self.name, var);
        polar::TyKind::Zero::<Pos>.intern(pcx)
    }
}

impl Check for Expr {
    fn check<'c>(&self, pcx: &'c polar::Context<'c>, cx: &mut Context) -> polar::Ty<'c, Pos> {
        match self {
            Expr::Lit(lit) => lit.check(pcx, cx),
            Expr::Scope(scope) => {
                let outer = cx.names.clone();
                for stmt in &scope.body {
                    stmt.check(pcx, cx);
                }
                let ty = match &scope.tail {
                    Some(expr) => expr.check(pcx, cx),
                    None => polar::TyKind::Zero::<Pos>.intern(pcx),
                };
                cx.names = outer;
                ty
            }
        }
    }
}

impl Check for expr::Lit {
    fn check<'c>(&self, pcx: &'c polar::Context<'c>, cx: &mut Context) -> polar::Ty<'c, Pos> {
        match self {
            expr::Lit::Int(_) => polar::TyKind::I32::<Pos>.intern(pcx),
            expr::Lit::Struct(map) => {
                let fields = map
                    .0
                    .iter()
                    .map(|(&label, expr)| (label, expr.check(pcx, cx)))
                    .collect();
                polar::TyKind::Struct::<Pos>(fields).intern(pcx)
            }
        }
    }
}
