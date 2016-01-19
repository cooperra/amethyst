//! The most basic Amethyst example.

// ECS

type Entity = u64;

#[derive(Debug)]
struct Entities {
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

use std::any::Any;

type Component = (Entity, Box<Any>);

#[derive(Debug)]
struct World {
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
}

// Game

extern crate amethyst;

use amethyst::{Application, Duration, State, Trans};

struct Example;

struct Blah {
    x: i32,
    y: i32,
}

impl State for Example {
    fn on_start(&mut self) {
        println!("Begin!");
    }

    fn update(&mut self, _delta: Duration) -> Trans {
        let mut ents = World::new();
        let ent = ents.create_entity();
        let ent2 = ents.create_entity();

        ents.insert_component(ent2, Blah { x: 0, y: 0 });
        ents.insert_component(ent, Blah { x: 0, y: 0 });
        ents.destroy_entity(ent2);

        println!("{:#?}", ents);
        println!("Hello from Amethyst!");

        Trans::Quit
    }

    fn on_stop(&mut self) {
        println!("End!");
    }
}

fn main() {
    let mut game = Application::new(Example);
    game.run();
}
