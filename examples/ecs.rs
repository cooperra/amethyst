//! The most basic Amethyst example.

extern crate amethyst;

use amethyst::{Application, Duration, State, Trans};

// ECS

type Entity = u64;

struct Entities {
    dead: Vec<Entity>,
    next_id: Entity,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            dead: Vec::new(),
            next_id: 0,
        }
    }

    pub fn create(&mut self) -> Entity {
        if let Some(id) = self.dead.pop() {
            return id;
        }

        let new_entity = self.next_id;
        self.next_id += 1;

        new_entity
    }

    pub fn destroy(&mut self, entity: Entity) {
        if entity < self.next_id {
            self.dead.push(entity);
        }
    }
}

struct World {
    entities: Entities,
}

struct Test;

// Game

struct Example;

impl State for Example {
    fn on_start(&mut self) {
        println!("Begin!");
    }

    fn update(&mut self, _delta: Duration) -> Trans {
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
