use crate::component::Component;
use core::marker::PhantomData;

/// System must be implemented by any object that wants to interact with
/// components. Objects implementing `System` are used to manipulate, and act on
/// the data stored by components.
///
/// The hierarchy of a system is as follows:
///
/// System -> Query -> Join<Read/Write<Component>, StatementN...>
pub trait System {
    type Query: Query;

    fn update(components: Self::Query);
}

/// A `Query` is a group of statements that determine on which components the
/// system will operate.
pub trait Query {}
impl<T> Query for Join<T> {}

/// A `Statement` defines a single `Read` or `Write` action for a `Component`.
pub trait Statement {
    type Component: Component;
}

/// `Read` marks a `Component` within a `Query` as read-only.
#[derive(Debug)]
pub struct Read<C: Component>(PhantomData<C>);

/// `Write` marks a `Component` within a `Query` as read-and-write.
#[derive(Debug)]
pub struct Write<C: Component>(PhantomData<C>);

impl<C: Component> Statement for Read<C> {
    type Component = C;
}

impl<C: Component> Statement for Write<C> {
    type Component = C;
}

/// `Join` takes one or more `Statement`s, and joins all requested `Component`s
/// together to be used in an iterator.
pub struct Join<T>(PhantomData<T>);

// TODO: This is a temporary stub implementation for a `Query` consisting of a
//       `Join` of two `Statement`s. In the future, this will be implemented for
//       A..Z using a macro.
impl<A, B> Iterator for Join<(A, B)>
where
    A: Statement,
    B: Statement,
{
    type Item = (A::Component, B::Component);

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}
