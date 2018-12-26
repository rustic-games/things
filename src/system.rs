use crate::{component::Component,
            store::{ComponentStore, Store}};
use core::{any::TypeId,
           iter::{FilterMap, Zip}};
use rustc_hash::FxHashMap as HashMap;

/// System must be implemented by any object that wants to interact with
/// components. Objects implementing `System` are used to manipulate, and act on
/// the data stored by components.
///
/// The hierarchy of a system is as follows:
///
/// System -> Query -> Join<Read/Write<Component>, StatementN...>
pub trait System<'a> {
    type Query: Query<'a>;

    fn update(components: <Self::Query as Query<'a>>::Iter);
}

/// A `Query` is a group of statements that determine on which set of components
/// the system will operate.
pub trait Query<'a> {
    type Iter: Iterator;

    fn iter(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter;
}

/// A `Statement` defines a single `Read` or `Write` action for a `Component`
/// type. It is meant to return an iterator for that type.
pub trait Statement<'a> {
    type Component: Component;
    type Iter: ExactSizeIterator<Item = &'a Option<Self::Component>>;

    fn find(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter;
}

/// `Read` marks a `Component` within a `Query` as read-only.
pub struct Read<C: Component>(C);

/// `Write` marks a `Component` within a `Query` as read-and-write.
pub struct Write<C: Component>(C);

impl<'a, C: Component> Statement<'a> for Read<C> {
    type Component = C;
    type Iter = std::slice::Iter<'a, Option<Self::Component>>;

    fn find(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter {
        let id = TypeId::of::<C>();
        if let Some(store) = store.get(&id) {
            return store.as_store::<C>().unwrap().as_slice().iter();
        }

        (&[]).iter()
    }
}

// TODO: This is a temporary stub implementation for a `Query` consisting of a
//       `Join` of two `Statement`s. In the future, this will be implemented for
//       A..Z using a macro.
impl<'a, A, B> Query<'a> for (A, B)
where
    A: Statement<'a>,
    B: Statement<'a>,
{
    // TODO: simplify type signature
    #[allow(clippy::type_complexity)]
    type Iter = FilterMap<
        Zip<A::Iter, B::Iter>,
        fn(
            (&'a Option<A::Component>, &'a Option<B::Component>),
        ) -> Option<(&'a A::Component, &'a B::Component)>,
    >;

    fn iter(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter {
        let a = A::find(store);
        let b = B::find(store);

        a.zip(b)
            .filter_map(|(a, b)| match (a.as_ref(), b.as_ref()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            })
    }
}
