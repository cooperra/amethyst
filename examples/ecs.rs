//! Example ECS usage.

extern crate amethyst;

use amethyst::{Application, Duration, State, Trans};
use amethyst::ecs::World;

struct Example;

struct Position {
    x: f32,
    y: f32,
    z: f32,
}

impl State for Example {
    fn on_start(&mut self) {
        println!("Begin!");
    }

    fn update(&mut self, _delta: Duration) -> Trans {
        let mut world = World::new();
        let e = world.create_entity();

        world.insert_component(e, Position { x: 0.0, y: 0.0, z: 0.0 });

        println!("{:#?}", world);
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
