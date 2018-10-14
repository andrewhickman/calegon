use ty::automaton::Automaton;
use ty::polar::{Context, TyKind};
use variance::{Neg, Pos};

#[test]
fn test() {
    let ctx = Context::new();
    let ty = TyKind::Recursive(
        TyKind::Fn::<Pos>(
            TyKind::Fn::<Neg>(
                TyKind::Var::<Pos>(0).intern(&ctx),
                TyKind::Var::<Neg>(-1).intern(&ctx),
            ).intern(&ctx),
            TyKind::Var::<Pos>(-1).intern(&ctx),
        ).intern(&ctx),
    ).intern(&ctx);

    let nfa = Automaton::new(ty);
    println!("{:#?}", nfa);
}
