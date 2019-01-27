use crate::Component;
use core::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};
use std::cell::UnsafeCell;

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
    fn push(&mut self, position: usize, component: Self::Item) -> usize;

    /// Extracts a slice containing the entire vector.
    ///
    /// Since the store allows for "padding" to be inserted, to allow components
    /// of the same entity to be aligned across different stores, the returned
    /// values is of type `Option<Component>`. A `None` indicates that the
    /// entity stored in that position does not contain the component type of
    /// this store.
    fn as_slice(&self) -> &[Option<Self::Item>];

    /// Extracts a mutable slice of the entire vector.
    ///
    /// Since the store allows for "padding" to be inserted, to allow components
    /// of the same entity to be aligned across different stores, the returned
    /// values is of type `Option<Component>`. A `None` indicates that the
    /// entity stored in that position does not contain the component type of
    /// this store.
    unsafe fn as_mut_slice(&self) -> &mut [Option<Self::Item>];
}

pub trait ComponentStore: Downcast {}
impl_downcast!(ComponentStore);

impl ComponentStore {
    pub fn as_store<C: Component>(&self) -> Option<&DefaultStore<C>> {
        self.downcast_ref::<DefaultStore<C>>()
    }

    pub fn as_mut_store<C: Component>(&mut self) -> Option<&mut DefaultStore<C>> {
        self.downcast_mut::<DefaultStore<C>>()
    }
}

#[derive(Debug)]
pub struct DefaultStore<C: Component>(UnsafeCell<Vec<Option<C>>>);

impl<C: Component> ComponentStore for DefaultStore<C> {}

impl<C: Component> Default for DefaultStore<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: Component> Store for DefaultStore<C> {
    type Item = C;

    fn new() -> Self {
        Self(UnsafeCell::new(Vec::new()))
    }

    fn push(&mut self, position: usize, component: C) -> usize {
        let store = unsafe { &mut (*self.0.get()) };

        // A component is either pushed right after the last element, or one or
        // more `None`s are pushed before the actual component is pushed.
        if store.len() < position {
            // This adds `None`s to all positions except the one where we want
            // to store the pushed `Component`. This is the same as
            // `resize(position, None)`, except that isn't allowed somehow,
            // because it requires `Copy`.
            store.resize_default(position);
        }

        store.push(Some(component));
        store.len()
    }

    fn as_slice(&self) -> &[Option<C>] {
        let store = unsafe { &(*self.0.get()) };
        store.as_slice()
    }

    unsafe fn as_mut_slice(&self) -> &mut [Option<C>] {
        let store = &mut (*self.0.get());
        store.as_mut_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestStore<C: Component>(UnsafeCell<Vec<Option<C>>>);
    impl<C: Component> ComponentStore for TestStore<C> {}

    #[derive(Debug)]
    struct C;

    #[rustfmt::skip]
    impl<C: Component> Store for TestStore<C> {
        type Item = C;

        fn new() -> Self { TestStore(UnsafeCell::new(Vec::new())) }
        fn push(&mut self, _: usize, _: C) -> usize { 0 }
        fn as_slice(&self) -> &[Option<C>] { unsafe { &(*self.0.get()) }.as_slice() }
        unsafe fn as_mut_slice(&self) -> &mut [Option<C>] { { &mut (*self.0.get()) }.as_mut_slice() }
     }

    #[rustfmt::skip]
    impl<C: Component> Default for TestStore<C> {
        fn default() -> Self { Self::new() }
    }

    #[test]
    fn test_implicit_component_trait() {
        let mut cs = TestStore::<C>::new();

        assert_eq!(cs.push(0, C), 0);
    }
}
