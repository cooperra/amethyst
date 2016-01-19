//! The entity-component-system framework.

mod entity;
mod world;

pub use self::entity::{Component, Entity};
pub use self::world::World;
