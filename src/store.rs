use crate::Component;
use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

/// `Store` must be implemented by any object that wants to store
/// [`Component`]s.
///
/// With this trait implemented, the object can be registered as the storage
/// engine for a single component type. The implementation can abstract away the
/// internal details on how data is stored (such as ["array of structs" or
/// "struct of arrays"][s]), allowing each implementation to provide optimum
/// performance for different use-cases.
///
/// This library provides several store implementations to start with, but more
/// specialised implementations can be used when required.
///
/// [s]: https://en.wikipedia.org/wiki/AOS_and_SOA
pub trait Store: Sized + Default + Debug {
    type Item: Component;

    /// `new` initialises a new (empty) store.
    fn new() -> Self;

    /// `push` takes a single component, and adds it to the store.
    ///
    /// The `usize` value returned by the method indicates the position in the
    /// store at which the component is stored. This can be used to later
    /// retrieve a specific component from the store.
    fn push(&mut self, component: Self::Item) -> usize;
}

pub trait ComponentStore: Downcast {}
impl_downcast!(ComponentStore);

impl ComponentStore {
    pub fn as_store<C: Component>(&self) -> Option<&DefaultStore<C>> {
        self.downcast_ref::<DefaultStore<C>>()
    }

    pub fn as_store_mut<C: Component>(&mut self) -> Option<&mut DefaultStore<C>> {
        self.downcast_mut::<DefaultStore<C>>()
    }
}

#[derive(Debug)]
pub struct DefaultStore<C: Component> {
    data: Vec<C>,
}

impl<C: Component> ComponentStore for DefaultStore<C> {}

impl<C: Component> Default for DefaultStore<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Component> Store for DefaultStore<C> {
    type Item = C;

    fn new() -> Self {
        Self { data: Vec::new() }
    }

    fn push(&mut self, component: Self::Item) -> usize {
        self.data.push(component);
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::marker::PhantomData;

    #[derive(Debug)]
    struct TestStore<C: Component>(PhantomData<C>);
    impl<C: Component> ComponentStore for TestStore<C> {}

    #[derive(Debug)]
    struct C;

    #[rustfmt::skip]
    impl<C: Component> Store for TestStore<C> {
        type Item = C;

        fn new() -> Self { TestStore(PhantomData) }
        fn push(&mut self, _: Self::Item) -> usize { 0 }
     }

    #[rustfmt::skip]
    impl<C: Component> Default for TestStore<C> {
        fn default() -> Self { Self::new() }
    }

    #[test]
    fn test_implicit_component_trait() {
        let mut cs = TestStore::<C>::new();

        assert_eq!(cs.push(C), 0);
    }
}
