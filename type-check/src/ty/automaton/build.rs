use ty::automaton::head::{Constructor, ConstructorSet};
use ty::automaton::{Automaton, FieldAlphabet, State, StateId};
use ty::polar::{Ty, TyNeg, TyPos, Visitor};
use ty::Fields;
use variance::Polarity;

struct BuildVisitor {
    states: Vec<State>,
    recs: Vec<StateId>,
}

impl BuildVisitor {
    fn next_id(&mut self) -> StateId {
        self.states.len() as StateId
    }

    fn current(&mut self) -> &mut State {
        self.states.last_mut().expect("no current state")
    }

    fn add_constructor(&mut self, con: Constructor) {
        let mut cons = ConstructorSet::singleton(con);
        match self.current().pol {
            Polarity::Pos => self.current().cons.join(&mut cons),
            Polarity::Neg => self.current().cons.meet(&mut cons),
        }
    }
}

impl Visitor for BuildVisitor {
    fn visit_var(&mut self, pol: Polarity, idx: u32) {
        assert_eq!(self.current().pol, pol);

        let id = self.recs[self.recs.len() - idx as usize];
        self.add_constructor(Constructor::Var(id));
    }

    fn visit_join(&mut self, lhs: &TyPos, rhs: &TyPos) {
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_meet(&mut self, lhs: &TyNeg, rhs: &TyNeg) {
        lhs.accept(self);
        rhs.accept(self);
    }

    fn visit_i32(&mut self, pol: Polarity) {
        assert_eq!(self.current().pol, pol);

        self.add_constructor(Constructor::I32);
    }

    fn visit_fn_pos(&mut self, domain: &TyNeg, range: &TyPos) {
        let idx = self.states.len() - 1;
        self.add_constructor(Constructor::Fn);

        let d = self.next_id();
        self.states[idx].trans.push((FieldAlphabet::Domain, d));
        self.states.push(State::empty(Polarity::Neg));
        domain.accept(self);

        let r = self.next_id();
        self.states[idx].trans.push((FieldAlphabet::Range, r));
        self.states.push(State::empty(Polarity::Pos));
        range.accept(self);
    }

    fn visit_fn_neg(&mut self, domain: &TyPos, range: &TyNeg) {
        let idx = self.states.len() - 1;
        self.add_constructor(Constructor::Fn);

        let d = self.next_id();
        self.states[idx].trans.push((FieldAlphabet::Domain, d));
        self.states.push(State::empty(Polarity::Pos));
        domain.accept(self);

        let r = self.next_id();
        self.states[idx].trans.push((FieldAlphabet::Range, r));
        self.states.push(State::empty(Polarity::Neg));
        range.accept(self);
    }

    fn visit_struct_pos(&mut self, fields: &Fields<TyPos>) {
        let idx = self.states.len() - 1;
        self.add_constructor(Constructor::Struct(fields.labels()));

        for &(label, ref ty) in fields.get() {
            let l = self.next_id();
            self.states[idx].trans.push((FieldAlphabet::Label(label), l));
            ty.accept(self);
        }
    }

    fn visit_struct_neg(&mut self, fields: &Fields<TyNeg>) {
        let idx = self.states.len() - 1;
        self.add_constructor(Constructor::Struct(fields.labels()));

        for &(label, ref ty) in fields.get() {
            let l = self.next_id();
            self.states[idx].trans.push((FieldAlphabet::Label(label), l));
            ty.accept(self);
        }
    }

    fn visit_recursive_pos(&mut self, ty: &TyPos) {
        let id = self.next_id();
        self.recs.push(id);
        ty.accept(self);
        self.recs.pop();
    }

    fn visit_recursive_neg(&mut self, ty: &TyNeg) {
        let id = self.next_id();
        self.recs.push(id);
        ty.accept(self);
        self.recs.pop();
    }
}

impl Automaton {
    pub fn new(ty: &Ty) -> Self {
        let mut visitor = BuildVisitor {
            recs: Vec::new(),
            states: vec![State::empty(ty.polarity())],
        };

        ty.accept(&mut visitor);

        Automaton {
            states: visitor.states,
        }
    }
}
