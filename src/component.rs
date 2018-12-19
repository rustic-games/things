use core::fmt::Debug;

/// The `Component` trait is a marker trait that allows any object implementing
/// it to be stored in a component store.
pub trait Component: 'static + Debug {}

/// `Component` is automatically implemented for any object that has both a
/// static lifetime, and implements `Debug`.
impl<T> Component for T where T: 'static + Debug {}

/// A collection of one or more components should implement this trait to be
/// able to manipulate each individual component, or store them for later use.
pub trait ComponentCollection {}

impl<A: Component> ComponentCollection for (A,) {}
impl<A: Component, B: Component> ComponentCollection for (A, B) {}

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
