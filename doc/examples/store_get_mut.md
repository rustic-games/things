# Examples

```rust
#[derive(Debug)]
struct ComponentStore(anymap::Map);

impl Store for ComponentStore {
    fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> {
        self.0.get_mut::<Vec<C>>().map(|vec| vec.as_mut_slice())
    }

    // other implementations hidden...
    # fn register<C: Component>(&mut self) { self.0.insert(Vec::<C>::new()); }
    # fn new() -> Self { ComponentStore(anymap::Map::new()) }
    # fn accepts<C: Component>(&self) -> bool { true }
    # fn get<C: Component>(&self) -> Option<&[C]> { None }
    # fn push<C: Component>(&mut self, component: C) -> Result<(), String> { Ok(()) }
}

# use things::{Component, Store};
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

After implementing `get_mut`, we [`register`] the component type [`u32`] and
[`push`] a component into the store:

```rust
# fn main() {
let mut store = ComponentStore::new();

store.register::<u32>();
store.push(1_u32);
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
#     fn push<C: Component>(&mut self, component: C) -> Result<(), String> {
#         self.0.get_mut::<Vec<C>>().unwrap().push(component); Ok(())
#     }
#     fn get<C: Component>(&self) -> Option<&[C]> { None }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
```

After we've done so, we can manipulate the component using `get_mut`:

```rust
# fn main() {
# let mut store = ComponentStore::new();
# store.register::<u32>();
# store.push(1_u32);

store.get_mut::<u32>().unwrap().iter_mut().for_each(|i| *i += 1);

assert_eq!(store.get::<u32>().unwrap()[0], 2);
# }

# use things::{Component, Store};
#
# #[derive(Debug)]
# struct ComponentStore(anymap::Map);
#
# impl Store for ComponentStore {
#     fn new() -> Self { ComponentStore(anymap::Map::new()) }
#     fn register<C: Component>(&mut self) { self.0.insert(Vec::<C>::new()); }
#     fn accepts<C: Component>(&self) -> bool { true }
#     fn push<C: Component>(&mut self, component: C) -> Result<(), String> {
#         self.0.get_mut::<Vec<C>>().unwrap().push(component); Ok(())
#     }
#     fn get<C: Component>(&self) -> Option<&[C]> {
#         self.0.get::<Vec<C>>().map(|vec| vec.as_slice())
#     }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> {
#         self.0.get_mut::<Vec<C>>().map(|vec| vec.as_mut_slice())
#     }
# }
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

[`register`]: Store::register
[`push`]: Store::push
