#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[macro_use]
extern crate log;
extern crate env_logger;

#[cfg(test)] mod tests;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    env_logger::init(); // Comment this out and it will run.
    println!("1");
    let rocket = rocket::ignite();
    println!("2");
    let rocket = rocket.mount("/", routes![hello]);
    println!("3"); // This is the last thing that gets printed.
    rocket.launch();
}
