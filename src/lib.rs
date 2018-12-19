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

use crate::component::ComponentCollection;
pub use crate::{component::Component, store::Store};
