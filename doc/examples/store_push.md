# Examples

After using the [`register`] method to accept components of a specified type,
you can then define `push` to add new instances of that type to the store.

```rust
#[derive(Debug)]
struct ComponentStore(anymap::Map);

impl Store for ComponentStore {
    fn push<C: Component>(&mut self, component: C) -> Result<(), String> {
        if let Some(component_store) = self.0.get_mut::<Vec<C>>() {
            component_store.push(component);
            return Ok(());
        }

        Err("component type not registered".to_owned())
    }

    // other implementations hidden...
    # fn new() -> Self { ComponentStore(anymap::Map::new()) }
    # fn register<C: Component>(&mut self) {}
    # fn accepts<C: Component>(&self) -> bool { true }
    # fn get<C: Component>(&self) -> Option<&[C]> { None }
    # fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
}

# use things::{Component, Store};
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

Since we've already registered `usize` as a component type in the [`register`]
example, multiple instances of that component can be pushed into the store:

```rust
# fn main() {
# let mut store = ComponentStore::new();
# store.register::<usize>();
assert!(store.push(3_usize).is_ok());
assert!(store.push(12_usize).is_ok());
# }

# use things::{Component, Store};
#
# #[derive(Debug)]
# struct ComponentStore(anymap::Map);
#
# impl Store for ComponentStore {
#     fn push<C: Component>(&mut self, component: C) -> Result<(), String> {
#         if let Some(component_store) = self.0.get_mut::<Vec<C>>() {
#             component_store.push(component);
#             return Ok(());
#         }
#
#         Err("component type not registered".to_owned())
#     }
#
#     fn new() -> Self { ComponentStore(anymap::Map::new()) }
#     fn register<C: Component>(&mut self) { self.0.insert(Vec::<C>::new()); }
#     fn accepts<C: Component>(&self) -> bool { true }
#     fn get<C: Component>(&self) -> Option<&[C]> { None }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

Trying to push a component type that has not yet been registered, will result in
an error:

```rust
# fn main() {
# let mut store = ComponentStore::new();
let error = Err("component type not registered".to_owned());

assert_eq!(store.push(String::from("hello")), error);
# }

# use things::{Component, Store};
#
# #[derive(Debug)]
# struct ComponentStore(anymap::Map);
#
# impl Store for ComponentStore {
#     fn push<C: Component>(&mut self, component: C) -> Result<(), String> {
#         if let Some(component_store) = self.0.get_mut::<Vec<C>>() {
#             component_store.push(component);
#             return Ok(());
#         }
#
#         Err("component type not registered".to_owned())
#     }
#
#     fn new() -> Self { ComponentStore(anymap::Map::new()) }
#     fn register<C: Component>(&mut self) {}
#     fn accepts<C: Component>(&self) -> bool { true }
#     fn get<C: Component>(&self) -> Option<&[C]> { None }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

[`register`]: Store::register
