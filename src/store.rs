use crate::Component;

/// `Store` must be implemented by any object that wants to store components.
///
/// With this trait implemented, the object can be registered as the storage
/// engine of the ECS instance. The implementation can abstract away the
/// internal details on how data is stored (such as ["array of structs" or
/// "struct of arrays"][s]), allowing each implementation to provide optimum
/// performance for different use-cases.
///
/// This library provides several store implementations to start with, but more
/// specialised implementations can be used when required.
///
/// [s]: https://en.wikipedia.org/wiki/AOS_and_SOA
pub trait Store: Sized + Default + core::fmt::Debug {
    /// `new` initialises a new (empty) store.
    fn new() -> Self;

    /// `register` configures the store to accept any object type that conforms
    /// to the [`Component`] trait as a valid component.
    ///
    /// This method has to be called before a new component of any type is
    /// [`push`]ed into the store.
    ///
    /// [`push`]: Store::push
    fn register<C: Component>(&mut self);

    /// `push` takes an instance of a component type, and adds it to the store.
    ///
    /// This method returns an error if the component type has not yet been
    /// [`register`]ed with the store.
    ///
    /// [`register`]: Store::register
    fn push<C: Component>(&mut self, component: C) -> Result<(), String>;

    /// `accepts` validates that a _type of component_ has been registered with
    /// the store, allowing the store to take any instance of that component
    /// type.
    ///
    /// Note that this is a _type check_. This method does not check if a
    /// specific instance of a component has been added to the store.
    fn accepts<C: Component>(&self) -> bool;

    /// `get` returns an immutable slice of all components in the store,
    /// matching the provided component type signature.
    ///
    /// The return value should be [`None`] if the component type is not
    /// [`register`]ed with the store, and should by an empty [`slice`] if there
    /// are no instances of a registered component type stored.
    ///
    /// [`slice`]: std::slice
    fn get<C: Component>(&self) -> Option<&[C]>;

    /// `get_mut` returns a mutable slice of all components in the store,
    /// matching the provided component type signature.
    ///
    /// The return value should be [`None`] if the component type is not
    /// [`register`]ed with the store, and should by an empty [`slice`] if there
    /// are no instances of a registered component type stored.
    ///
    /// [`slice`]: std::slice
    fn get_mut<C: Component>(&mut self) -> Option<&mut [C]>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct ComponentStore;

    #[rustfmt::skip]
    impl Store for ComponentStore {
        fn new() -> Self { ComponentStore }
        fn register<C: Component>(&mut self) {}
        fn accepts<C: Component>(&self) -> bool { true }
        fn get<C: Component>(&self) -> Option<&[C]> { None }
        fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
        fn push<C: Component>(&mut self, _: C) -> Result<(), String> { Ok(()) }
    }

    #[test]
    fn test_implicit_component_trait() {
        let mut cs = ComponentStore::new();
        cs.register::<usize>();
    }
}
