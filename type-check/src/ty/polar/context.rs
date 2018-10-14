use typed_arena::Arena;

use ty::polar::{Ty, TyKind};
use variance::{Neg, Pos};

pub struct Context<'c> {
    // TODO: caching
    arena: Arena<InternedTy<'c>>,
}

#[derive(Debug)]
enum InternedTy<'c> {
    Neg(TyKind<'c, Neg>),
    Pos(TyKind<'c, Pos>),
}

impl<'c> Context<'c> {
    pub(in ty::polar) fn intern_pos(&'c self, ty: TyKind<'c, Pos>) -> Ty<'c, Pos> {
        self.arena.alloc(InternedTy::Pos(ty)).unwrap_pos()
    }

    pub(in ty::polar) fn intern_neg(&'c self, ty: TyKind<'c, Neg>) -> Ty<'c, Neg> {
        self.arena.alloc(InternedTy::Neg(ty)).unwrap_neg()
    }
}

impl<'c> InternedTy<'c> {
    fn unwrap_pos(&'c self) -> Ty<'c, Pos> {
        match self {
            InternedTy::Neg(_) => unreachable!(),
            InternedTy::Pos(kind) => Ty { kind, pol: Pos },
        }
    }

    fn unwrap_neg(&'c self) -> Ty<'c, Neg> {
        match self {
            InternedTy::Neg(kind) => Ty { kind, pol: Neg },
            InternedTy::Pos(_) => unreachable!(),
        }
    }
}
