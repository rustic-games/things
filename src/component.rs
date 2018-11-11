use core::fmt::Debug;

/// The `Component` trait is a marker trait that allows any object implementing
/// it to be stored in a component store.
pub trait Component: 'static + Debug {}

/// `Component` is automatically implemented for any object that has both a
/// static lifetime, and implements `Debug`.
impl<T> Component for T where T: 'static + Debug {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implicit_component_trait() {
        #[derive(Debug)]
        struct C;

        let _: Box<Component> = Box::new(C);
    }
}
