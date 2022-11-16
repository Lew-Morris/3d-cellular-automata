use bevy::prelude::*;

fn main() {
    // println!("Hello, world!");
    App::new()
        .add_startup_system(startup)
        .run();
}

fn startup() {
    println!("Hello world!");
}
