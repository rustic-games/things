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
#![feature(vec_resize_default)]

mod borrow;
mod component;
mod entity;
mod store;
mod system;

pub use crate::{borrow::BorrowError,
                component::Component,
                entity::Entity,
                store::Store,
                system::{Query, Read, System, Write}};
use crate::{borrow::{RegisterBorrow, RuntimeBorrow},
            component::ComponentCollection,
            store::ComponentStore};
use generational_arena::Arena;
use parking_lot::Mutex;
use rustc_hash::FxHashMap as HashMap;
use std::any::TypeId;

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

    /// component_cursor keeps track of the next position in the stores where
    /// the components of the next added entity should be stored. This is used
    /// to keep the components aligned across their respective stores, to know
    /// which components belong to a single `Entity`.
    component_cursor: usize,

    /// entity_component_references is a map of [`Component`] locations, as they
    /// relate to an [`Entity`]. This map allows finding all components
    /// belonging to a single entity.
    ///
    /// Because multiple components of the same type can belong to a single
    /// entity, a record is kept of the maximum number of components of the same
    /// type an entity has. The first [`usize`] is the position of the first
    /// component of each type in the stores. The second [`usize`] is the
    /// maximum number of components per type stored for this entity. If it is
    /// `1`, then each component type is only used once for the entity. Anything
    /// above 1 means that one or more component types are represented more than
    /// one time. The store will then query "up to x" for each type, and get
    /// back None if a type has reached its maximum members for the given
    /// entity.
    entity_component_references: HashMap<Entity, (usize, usize)>,

    runtime_borrow: Mutex<RuntimeBorrow>,
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
            component_stores: HashMap::default(),
            component_cursor: 0,
            entity_component_references: HashMap::default(),
            runtime_borrow: Mutex::new(RuntimeBorrow::new()),
        }
    }

    pub fn execute_system<'a, S: System<'a>>(&'a mut self) -> Result<(), BorrowError>
    where
        S::Query: Query<'a>,
        <S::Query as Query<'a>>::Borrow: RegisterBorrow,
    {
        self.borrow_and_validate::<<S::Query as Query<'a>>::Borrow>()?;

        let query = S::Query::iter(&self.component_stores);

        S::update(query);
        Ok(())
    }

    pub fn create_entity<CC: ComponentCollection>(&mut self, components: CC) {
        let entity = Entity::from(self.entities.insert(()));
        let result = components.store(&mut self.component_stores, self.component_cursor);

        self.component_cursor += result.len;
        self.entity_component_references
            .insert(entity, (result.position, result.len));
    }

    fn borrow_and_validate<Borrow: RegisterBorrow>(&self) -> Result<(), BorrowError> {
        let mut borrow = self.runtime_borrow.lock();
        borrow.push_access::<Borrow>()?;
        borrow.validate()
    }
}
