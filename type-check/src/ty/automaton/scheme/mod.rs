mod flow;

use ty::automaton::build::build;
use ty::automaton::scheme::flow::FlowSet;
use ty::automaton::state::{State, StateId};
use ty::polar;
use variance::AsPolarity;

pub struct Scheme {
    states: Vec<State>,
    //flows: FlowSet,
    tys: Vec<StateId>,
    expr: StateId,
}

impl Scheme {
    pub fn new<'c, P, I>(tys: I, expr: polar::Ty<'c, P>) -> Self
    where
        P: AsPolarity,
        I: IntoIterator<Item = polar::Ty<'c, P::Neg>>,
    {
        let mut states = Vec::new();
        let expr = build(&mut states, expr);
        let tys = tys.into_iter().map(|ty| build(&mut states, ty)).collect();
        //let flows = FlowSet::new(&[states]);
        Scheme {
            states,
            expr,
            tys,
            //   flows,
        }
    }
}
