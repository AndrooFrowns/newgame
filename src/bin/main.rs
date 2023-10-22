#![deny(clippy::all, clippy::pedantic)]
use newgame::core::app;

struct MyApp {}

impl app::App for MyApp {}

fn main() {
    println!("Hello, world!");

    let local_app: Box<dyn app::App> = Box::new(MyApp {});

    local_app.run();
}
