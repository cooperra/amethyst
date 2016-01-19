use std::any::Any;

pub type Entity = u64;

pub type Component = (Entity, Box<Any>);

#[derive(Debug)]
pub struct Entities {
    alive: Vec<Entity>,
    dead: Vec<Entity>,
    next_id: Entity,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            alive: Vec::new(),
            dead: Vec::new(),
            next_id: 0,
        }
    }

    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.dead.pop() {
            self.alive.push(id.clone());
            return id;
        }

        let new_entity = self.next_id;
        self.alive.push(new_entity.clone());
        self.next_id += 1;

        new_entity
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.alive.iter().chain(&self.dead).filter(|&e| *e == entity).count() == 1
    }

    pub fn destroy(&mut self, entity: Entity) {
        if self.is_alive(entity) {
            self.alive.retain(|&e| e != entity);
            self.dead.push(entity);
        }
    }
}
