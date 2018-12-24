use crate::store::{ComponentStore, DefaultStore, Store};
use core::fmt::Debug;
use rustc_hash::FxHashMap as HashMap;
use std::any::TypeId;

/// The `Component` trait is a marker trait that allows any object implementing
/// it to be stored in a component store.
pub trait Component: 'static + Debug {}

/// `Component` is automatically implemented for any object that has both a
/// static lifetime, and implements `Debug`.
impl<T> Component for T where T: 'static + Debug {}

/// A collection of one or more components should implement this trait to be
/// able to manipulate each individual component, or store them for later use.
pub trait ComponentCollection {
    fn store(self, stores: &mut HashMap<TypeId, Box<ComponentStore>>, cursor: usize)
        -> StoreResult;
}

pub struct StoreResult {
    pub position: usize,
    pub len: usize,
}

impl<A, B> ComponentCollection for (A, B)
where
    A: Component,
    B: Component,
{
    fn store(
        self,
        stores: &mut HashMap<TypeId, Box<ComponentStore>>,
        cursor: usize,
    ) -> StoreResult {
        let id_a = TypeId::of::<A>();
        let id_b = TypeId::of::<B>();

        let store = stores
            .entry(id_a)
            .or_insert_with(|| Box::new(DefaultStore::<A>::default()))
            .as_mut_store::<A>()
            .unwrap();

        let position_a = store.push(cursor, self.0);

        let store = stores
            .entry(id_b)
            .or_insert_with(|| Box::new(DefaultStore::<B>::default()))
            .as_mut_store::<B>()
            .unwrap();

        let position_b = store.push(cursor, self.1);

        // Take the lowest inserted position as the starting point for the
        // components of this entity.
        //
        // If components `Position` and `Velocity` are inserted, then both will
        // be at the same position, but if two `Position` components are stored
        // for a single entity, one will come after the other, so we have to
        // keep track of both the starting position in the store, and the count
        // of the component which is used the most for this entity.
        let position = std::cmp::min(position_a, position_b);
        let len = if position_a == position_b {
            1
        } else {
            std::cmp::max(position_a, position_b) - position
        };

        StoreResult { position, len }
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
