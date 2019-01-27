use crate::{component::Component,
            store::{ComponentStore, Store}};
use core::{any::TypeId,
           iter::{FilterMap, Zip}};
use rustc_hash::FxHashMap as HashMap;

/// System must be implemented by any object that wants to interact with
/// components. Objects implementing `System` are used to manipulate, and act on
/// the data stored by components.
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

/// A `Reader` defines a single `Read` action for a component type. Its `find`
/// operation returns an iterator over all immutable references of the given
/// [`Component`].
pub trait Reader<'a> {
    type Component: Component;
    type Iter: ExactSizeIterator<Item = &'a Option<Self::Component>>;

    fn find(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter;
}

/// A `Writer` defines a single `Write` action for a component type. Its `find`
/// operation returns an iterator over all mutable references of the given
/// [`Component`].
pub trait Writer<'a> {
    type Component: Component;
    type Iter: ExactSizeIterator<Item = &'a mut Option<Self::Component>>;

    fn find(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter;
}

/// `Read` marks a `Component` within a `Query` as read-only.
pub struct Read<C: Component>(C);

/// `Write` marks a `Component` within a `Query` as read-and-write.
pub struct Write<C: Component>(C);

impl<'a, C: Component> Reader<'a> for Read<C> {
    type Component = C;
    type Iter = std::slice::Iter<'a, Option<Self::Component>>;

    fn find(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter {
        let id = TypeId::of::<C>();
        match store.get(&id) {
            Some(store) => store.as_store::<C>().unwrap().as_slice().iter(),
            None => (&[]).iter(),
        }
    }
}

// FIXME: This implementation requires a runtime borrow-checker to make sure
//        only one mutable reference can be fetched to any given [`Component`]
//        at any given time.
impl<'a, C: Component> Writer<'a> for Write<C> {
    type Component = C;
    type Iter = std::slice::IterMut<'a, Option<Self::Component>>;

    fn find(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter {
        let id = TypeId::of::<C>();
        match store.get(&id) {
            Some(store) => unsafe { store.as_store::<C>().unwrap().as_mut_slice() }.iter_mut(),
            None => (&mut []).iter_mut(),
        }
    }
}

// TODO: This is a temporary stub implementation for a `Query` consisting of a
//       `Read`, and a `Write`. In the future, this will be implemented for A..Z
//       using a macro.
impl<'a, A, B> Query<'a> for (Read<A>, Write<B>)
where
    A: Component,
    B: Component,
{
    type Iter = FilterMap<
        Zip<<Read<A> as Reader<'a>>::Iter, <Write<B> as Writer<'a>>::Iter>,
        fn((&'a Option<A>, &'a mut Option<B>)) -> Option<(&'a A, &'a mut B)>,
    >;

    fn iter(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter {
        let a = <Read<A> as Reader<'a>>::find(store);
        let b = <Write<B> as Writer<'a>>::find(store);

        a.zip(b)
            .filter_map(|(x, y)| match (x.as_ref(), y.as_mut()) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            })
    }
}

// TODO: This is a temporary stub implementation for a `Query` consisting of two
//       `Read`s. In the future, this will be implemented for A..Z using a
//       macro.
impl<'a, A, B> Query<'a> for (Read<A>, Read<B>)
where
    A: Component,
    B: Component,
{
    type Iter = FilterMap<
        Zip<<Read<A> as Reader<'a>>::Iter, <Read<B> as Reader<'a>>::Iter>,
        fn((&'a Option<A>, &'a Option<B>)) -> Option<(&'a A, &'a B)>,
    >;

    fn iter(store: &'a HashMap<TypeId, Box<ComponentStore>>) -> Self::Iter {
        let a = <Read<A> as Reader<'a>>::find(store);
        let b = <Read<B> as Reader<'a>>::find(store);

        a.zip(b)
            .filter_map(|(a, b)| match (a.as_ref(), b.as_ref()) {
                (Some(a), Some(b)) => Some((a, b)),
                _ => None,
            })
    }
}
