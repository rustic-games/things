use crate::store::{ComponentStore, DefaultStore, Store};
use core::fmt::Debug;
use std::{any::TypeId, collections::HashMap};

/// The `Component` trait is a marker trait that allows any object implementing
/// it to be stored in a component store.
pub trait Component: 'static + Debug {}

/// `Component` is automatically implemented for any object that has both a
/// static lifetime, and implements `Debug`.
impl<T> Component for T where T: 'static + Debug {}

/// A collection of one or more components should implement this trait to be
/// able to manipulate each individual component, or store them for later use.
pub trait ComponentCollection {
    fn store(self, stores: &mut HashMap<TypeId, Box<ComponentStore>>) -> Vec<(TypeId, usize)>;
}

impl<A> ComponentCollection for (A,)
where
    A: Component,
{
    fn store(self, stores: &mut HashMap<TypeId, Box<ComponentStore>>) -> Vec<(TypeId, usize)> {
        let mut references = Vec::new();
        let id_a = TypeId::of::<A>();

        let position = stores
            .entry(id_a)
            .or_insert_with(|| Box::new(DefaultStore::<A>::default()))
            .as_store_mut::<A>()
            .unwrap()
            .push(self.0);

        references.push((id_a, position));

        references
    }
}

impl<A, B> ComponentCollection for (A, B)
where
    A: Component,
    B: Component,
{
    fn store(self, stores: &mut HashMap<TypeId, Box<ComponentStore>>) -> Vec<(TypeId, usize)> {
        let mut references = Vec::new();
        let id_a = TypeId::of::<A>();
        let id_b = TypeId::of::<B>();

        let position = stores
            .entry(id_a)
            .or_insert_with(|| Box::new(DefaultStore::<A>::default()))
            .as_store_mut::<A>()
            .unwrap()
            .push(self.0);

        references.push((id_a, position));

        let position = stores
            .entry(id_b)
            .or_insert_with(|| Box::new(DefaultStore::<B>::default()))
            .as_store_mut::<B>()
            .unwrap()
            .push(self.1);

        references.push((id_b, position));

        references
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_component_trait() {
        #[derive(Debug)]
        struct C;

        let _: Box<Component> = Box::new(C);
    }

    #[test]
    fn test_component_collection() {
        #[derive(Debug)]
        struct C;

        let _: Box<ComponentCollection> = Box::new((C, C));
    }

    #[test]
    fn test_component_collection_heterogeneous() {
        #[derive(Debug)]
        struct C;

        #[derive(Debug)]
        struct D(usize);

        let _: Box<ComponentCollection> = Box::new((C, D(1)));
    }
}
