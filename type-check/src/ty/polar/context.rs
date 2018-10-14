use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::BuildHasherDefault;

use seahash::SeaHasher;
use typed_arena::Arena;

use ty::polar::{Ty, TyKind};
use variance::{Neg, Pos};

pub struct Context<'c> {
    cache: RefCell<HashSet<&'c InternedTy<'c>, BuildHasherDefault<SeaHasher>>>,
    arena: Arena<InternedTy<'c>>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum InternedTy<'c> {
    Neg(TyKind<'c, Neg>),
    Pos(TyKind<'c, Pos>),
}

impl<'c> Context<'c> {
    pub fn new() -> Self {
        Context {
            cache: Default::default(),
            arena: Arena::new(),
        }
    }
}

impl<'c> TyKind<'c, Pos> {
    pub fn intern(self, ctx: &'c Context<'c>) -> Ty<'c, Pos> {
        let ty = InternedTy::Pos(self);
        if let Some(cached) = ctx.cache.borrow().get(&ty) {
            return cached.unwrap_pos();
        }
        let interned = ctx.arena.alloc(ty);
        ctx.cache.borrow_mut().insert(interned);
        interned.unwrap_pos()
    }
}

impl<'c> TyKind<'c, Neg> {
    pub fn intern(self, ctx: &'c Context<'c>) -> Ty<'c, Neg> {
        let ty = InternedTy::Neg(self);
        if let Some(cached) = ctx.cache.borrow().get(&ty) {
            return cached.unwrap_neg();
        }
        let interned = ctx.arena.alloc(ty);
        ctx.cache.borrow_mut().insert(interned);
        interned.unwrap_neg()
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
