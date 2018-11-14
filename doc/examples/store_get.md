# Examples

```rust
#[derive(Debug)]
struct ComponentStore(anymap::Map);

impl Store for ComponentStore {
    fn get<C: Component>(&self) -> Option<&[C]> {
        self.0.get::<Vec<C>>().map(|vec| vec.as_slice())
    }

    // other implementations hidden...
    # fn new() -> Self { ComponentStore(anymap::Map::new()) }
    # fn register<C: Component>(&mut self) {}
    # fn accepts<C: Component>(&self) -> bool { true }
    # fn push<C: Component>(&mut self, component: C) -> Result<(), String> { Ok(()) }
    # fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
}

# use things::{Component, Store};
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

After implementing `get`, we [`register`] two component types [`u32`] and
[`u64`], and [`push`] three components into the store:

```rust
# fn main() {
let mut store = ComponentStore::new();

store.register::<u32>();
store.register::<u64>();

store.push(1_u32);
store.push(2_u64); // note: u64
store.push(3_u32);
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
#     fn get<C: Component>(&self) -> Option<&[C]> { None }
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

We can now fetch those components using `get`:

```rust
# fn main() {
# let mut store = ComponentStore::new();
#
# store.register::<u32>();
# store.register::<u64>();
#
# store.push(1_u32);
# store.push(2_u64);
# store.push(3_u32);

let components = store.get::<u32>().unwrap();
assert_eq!(components[0], 1);
assert_eq!(components[1], 3);

let components = store.get::<u64>().unwrap();
assert_eq!(components[0], 2);
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
#     fn get_mut<C: Component>(&mut self) -> Option<&mut [C]> { None }
# }
#
# impl Default for ComponentStore {
#     fn default() -> Self { Self::new() }
# }
```

[`register`]: Store::register
[`push`]: Store::push
