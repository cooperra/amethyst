use super::entities::{Component, Entity, Entities};

use std::any::Any;

#[derive(Debug)]
pub struct World {
    entities: Entities,
    components: Vec<Component>,
}

impl World {
    pub fn new() -> World {
        World {
            components: Vec::new(),
            entities: Entities::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entities.create()
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.entities.destroy(entity);
        self.components.retain(|e| (*e).0 != entity);
    }

    pub fn insert_component<T: Any>(&mut self, entity: Entity, comp: T) {
        if self.entities.is_alive(entity) {
            self.components.push((entity, Box::new(comp)));
            self.components.sort_by(|next, prev| next.0.cmp(&prev.0));
        }
    }

    pub fn get_components(&self) -> &Vec<Component> {
        &self.components
    }

    pub fn get_components_mut(&mut self) -> &mut Vec<Component> {
        &mut self.components
    }
}
