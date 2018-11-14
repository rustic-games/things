# Examples

```rust
#[derive(Debug)]
struct ComponentStore(anymap::Map);

impl Store for ComponentStore {
    fn accepts<C: Component>(&self) -> bool {
        self.0.contains::<Vec<C>>()
    }

    // other implementations hidden...
    # fn new() -> Self { ComponentStore(anymap::Map::new()) }
    # fn register<C: Component>(&mut self) {}
    # fn push<C: Component>(&mut self, component: C) -> Result<(), String> { Ok(()) }
    # fn get<C: Component>(&self) -> Option<&[C]> { None }
    # fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
}

# use things::{Component, Store};
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

Using `accepts` without first [`register`]ing a component type will return
false (as opposed to the [`Err`] we got when trying to [`push`] an instance of
this type into the store):

```rust
# fn main() {
# let mut store = ComponentStore::new();
assert_eq!(store.accepts::<String>(), false)
# }

# use things::{Component, Store};
#
# #[derive(Debug)]
# struct ComponentStore(anymap::Map);
#
# impl Store for ComponentStore {
#     fn new() -> Self { ComponentStore(anymap::Map::new()) }
#     fn register<C: Component>(&mut self) {}
#     fn accepts<C: Component>(&self) -> bool { self.0.contains::<Vec<C>>() }
#     fn push<C: Component>(&mut self, component: C) -> Result<(), String> { Ok(()) }
#     fn get<C: Component>(&self) -> Option<&[C]> { None }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

After [`register`]ing, `true` is returned:

```rust
# fn main() {
# let mut store = ComponentStore::new();
store.register::<String>();
assert_eq!(store.accepts::<String>(), true)
# }

# use things::{Component, Store};
#
# #[derive(Debug)]
# struct ComponentStore(anymap::Map);
#
# impl Store for ComponentStore {
#     fn new() -> Self { ComponentStore(anymap::Map::new()) }
#     fn register<C: Component>(&mut self) { self.0.insert(Vec::<C>::new()); }
#     fn accepts<C: Component>(&self) -> bool { self.0.contains::<Vec<C>>() }
#     fn push<C: Component>(&mut self, component: C) -> Result<(), String> { Ok(()) }
#     fn get<C: Component>(&self) -> Option<&[C]> { None }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

[`register`]: Store::register
[`push`]: Store::push
