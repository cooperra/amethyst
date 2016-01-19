//! Example ECS usage.

extern crate amethyst;

use amethyst::{Application, Duration, State, Trans, World};

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
