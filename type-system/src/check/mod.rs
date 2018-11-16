use im::HashMap;
use int_hash::IntBuildHasher;
use syntax::ast::*;
use syntax::Symbol;

use automaton::Automaton;
use variance::Pos;
use {polar, var, Var};

type Env = HashMap<Symbol, Var, IntBuildHasher>;

struct Context {
    var: var::Context,
    auto: Automaton,
}

fn check_expr<'c>(
    lit: &Expr,
    pcx: &'c polar::Context<'c>,
    cx: &mut Context,
    env: Env,
) -> polar::Ty<'c, Pos> {
    unimplemented!()
}

fn check_lit<'c>(
    lit: &expr::Lit,
    pcx: &'c polar::Context<'c>,
    cx: &mut Context,
    env: Env,
) -> polar::Ty<'c, Pos> {
    match lit {
        expr::Lit::Int(_) => polar::TyKind::I32::<Pos>.intern(pcx),
        expr::Lit::Struct(map) => {
            let fields = map
                .0
                .iter()
                .map(|(&label, expr)| (label, check_expr(expr, pcx, cx, env.clone())))
                .collect();
            polar::TyKind::Struct::<Pos>(fields).intern(pcx)
        }
    }
}
