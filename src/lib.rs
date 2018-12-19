#![cfg_attr(feature = "doc", feature(external_doc))]

mod component;
mod store;

use crate::component::ComponentCollection;
pub use crate::{component::Component, store::Store};
