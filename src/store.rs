use crate::Component;
use core::fmt::Debug;

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
    type C: Component;

    /// `new` initialises a new (empty) store.
    fn new() -> Self;

    /// `push` takes a single component, and adds it to the store.
    ///
    /// The `usize` value returned by the method indicates the position in the
    /// store at which the component is stored. This can be used to later
    /// retrieve a specific component from the store.
    fn push(&mut self, component: Self::C) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::marker::PhantomData;

    #[derive(Debug)]
    struct ComponentStore<C: Component>(PhantomData<C>);

    #[derive(Debug)]
    struct C;

    #[rustfmt::skip]
    impl<C: Component> Store for ComponentStore<C> {
        type C = C;

        fn new() -> Self { ComponentStore(PhantomData) }
        fn push(&mut self, _: C) -> usize { 0 }
     }

    #[rustfmt::skip]
    impl<C: Component> Default for ComponentStore<C> {
        fn default() -> Self { Self::new() }
    }

    #[test]
    fn test_implicit_component_trait() {
        let mut cs = ComponentStore::<C>::new();

        assert_eq!(cs.push(C), 0);
    }
}
