//! Things is a safe, fast and secure Entity–Component–System library.
//!
//! Entity–Component–System (ECS) is an architectural pattern that is mostly
//! used in game development. ECS follows the composition over inheritance
//! principle that allows greater flexibility in defining entities where every
//! object in a game's scene is an entity (e.g. enemies, bullets, vehicles,
//! etc.). Every Entity consists of one or more components which add behaviour
//! or functionality. Therefore, the behaviour of an entity can be changed at
//! runtime by adding or removing components. This eliminates the ambiguity
//! problems of deep and wide inheritance hierarchies that are difficult to
//! understand, maintain and extend. Common ECS approaches are highly compatible
//! and often combined with data-oriented design techniques.
//!
//! see: https://en.wikipedia.org/wiki/Entity–component–system

#![cfg_attr(feature = "doc", feature(external_doc))]

mod component;
mod store;

pub use crate::{component::Component, store::Store};
use crate::{component::ComponentCollection, store::ComponentStore};
use generational_arena::Arena;
use std::{any::TypeId, collections::HashMap};

/// Things is the top-level object used to interact with an instance of the ECS
/// functionality.
pub struct Things {
    /// entities are stored in a generational index, using the
    /// `generational-arena` crate.
    entities: Arena<()>,

    /// component_stores is a map of stores, one store for each component type.
    /// The type ID of each component is used as the key of the map, to allow
    /// linking the components back to the entity.
    component_stores: HashMap<TypeId, Box<ComponentStore>>,
}

impl Default for Things {
    fn default() -> Self {
        Self::new()
    }
}

impl Things {
    pub fn new() -> Self {
        Things {
            entities: Arena::new(),
            component_stores: HashMap::new(),
        }
    }

    pub fn create_entity<CC: ComponentCollection>(&mut self, components: CC) {
        self.entities.insert(());
        components.store(&mut self.component_stores);
    }
}
