# Examples

`register` is implemented for `ComponentStore`.

```rust
#[derive(Debug)]
struct ComponentStore(anymap::Map);

impl Store for ComponentStore {
    fn register<C: Component>(&mut self) {
        self.0.insert(Vec::<C>::new());
    }

    // other implementations hidden...
    # fn new() -> Self { ComponentStore(anymap::Map::new()) }
    # fn accepts<C: Component>(&self) -> bool { true }
    # fn push<C: Component>(&mut self, _: C) -> Result<(), String> { Ok(()) }
    # fn get<C: Component>(&self) -> Option<&[C]> { None }
    # fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
}

# use things::{Component, Store};
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

Each component type gets its own [`Vec`], setting the `Vec<T>`'s `T` to the
provided component type `C`. The resulting vec is stored inside an instance of
[`anymap`]. For any `register`ed component type, one or more instances of that
component can then be [`push`]ed into the store.

_The example uses the [`anymap`] crate for its ease of storing different
component types. Stores optimised for performance would use a different
implementation._

```rust
# fn main() {
let mut store = ComponentStore::new();

store.register::<usize>()
# }

# use things::{Component, Store};
#
# #[derive(Debug)]
# struct ComponentStore(anymap::Map);
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
#
# impl Store for ComponentStore {
#     fn new() -> Self { ComponentStore(anymap::Map::new()) }
#     fn register<C: Component>(&mut self) { self.0.insert(Vec::<C>::new()); }
#     fn accepts<C: Component>(&self) -> bool { true }
#     fn push<C: Component>(&mut self, _: C) -> Result<(), String> { Ok(()) }
#     fn get<C: Component>(&self) -> Option<&[C]> { None }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
```

For this example, [`usize`] is used as a component type. It has a static
lifetime, and implements the [`Debug`] trait by default.

[`push`]: Store::push
[`Debug`]: core::fmt::Debug
[`anymap`]: https://docs.rs/crate/anymap
