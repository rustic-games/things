use crate::{Component, Read, Write};
use rustc_hash::FxHashSet as HashSet;
use std::{any::TypeId, error, fmt};

#[derive(Default)]
pub(crate) struct RuntimeBorrow {
    borrows: Vec<Borrow>,
}

impl RuntimeBorrow {
    pub(crate) fn new() -> Self {
        RuntimeBorrow {
            borrows: Vec::new(),
        }
    }

    pub(crate) fn push_access<R: RegisterBorrow>(&mut self) -> Result<(), BorrowError> {
        let borrow = R::register_borrow()?;
        self.borrows.push(borrow);
        Ok(())
    }

    pub(crate) fn validate(&self) -> Result<(), BorrowError> {
        let invalid = self.borrows.iter().enumerate().any(|(idx, borrow)| {
            let non_exclusive_internal_write = !borrow.writes.is_disjoint(&borrow.reads);

            let cross_borrow_non_exclusive_write = self.borrows.iter().skip(idx + 1).any(|other| {
                !borrow.writes.is_disjoint(&other.writes)
                    || !borrow.writes.is_disjoint(&other.reads)
            });

            non_exclusive_internal_write || cross_borrow_non_exclusive_write
        });

        match invalid {
            true => Err(BorrowError::InvalidBorrow),
            false => Ok(()),
        }
    }
}

#[derive(Default)]
pub struct Borrow {
    reads: HashSet<TypeId>,
    writes: HashSet<TypeId>,
}

impl Borrow {
    fn new() -> Self {
        Borrow {
            reads: HashSet::default(),
            writes: HashSet::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum BorrowError {
    InvalidBorrow,
}

use BorrowError::*;

impl fmt::Display for BorrowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InvalidBorrow => f.write_str("unable to borrow component"),
        }
    }
}

impl error::Error for BorrowError {
    fn description(&self) -> &str {
        match self {
            InvalidBorrow => "unable to borrow component",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

type BorrowResult = Result<Borrow, BorrowError>;

pub trait RegisterBorrow {
    /// Creates a new borrow
    fn register_borrow() -> BorrowResult;
}

impl<A, B> RegisterBorrow for (A, B)
where
    A: PushBorrow,
    B: PushBorrow,
{
    fn register_borrow() -> BorrowResult {
        let mut borrow = Borrow::new();

        A::push_borrow(&mut borrow)?;
        B::push_borrow(&mut borrow)?;

        Ok(borrow)
    }
}

pub trait PushBorrow {
    fn push_borrow(borrow: &mut Borrow) -> Result<(), BorrowError>;
}

impl<C: Component> PushBorrow for Read<C> {
    /// Multiple reads are always allowed and therefor we can always return
    /// true.
    fn push_borrow(borrow: &mut Borrow) -> Result<(), BorrowError> {
        borrow.reads.insert(TypeId::of::<C>());
        Ok(())
    }
}

impl<C: Component> PushBorrow for Write<C> {
    /// Only a single write borrow is allowed. The `HashSet` returns false if
    /// the `TypeId` of the component is already present in the borrow.
    fn push_borrow(borrow: &mut Borrow) -> Result<(), BorrowError> {
        match borrow.writes.insert(TypeId::of::<C>()) {
            true => Ok(()),
            false => Err(BorrowError::InvalidBorrow),
        }
    }
}
