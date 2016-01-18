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

    pub fn destroy(&mut self, entity: Entity) {
        if entity < self.next_id {
            self.alive.retain(|id| *id != entity);
            self.dead.push(entity);
        }
    }
}

use std::any::Any;

type Component = (Entity, Box<Any>);

#[derive(Debug)]
struct World {
    entities: Entities,
    pub components: Vec<Component>,
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

        ents.components.push((ent2, Box::new(Blah { x: 0, y: 0 })));
        ents.components.push((ent, Box::new(Blah { x: 0, y: 0 })));
        ents.components.sort_by(|x, y| {
            x.0.cmp(&y.0)
        });
        let ref view = ents.components;

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
