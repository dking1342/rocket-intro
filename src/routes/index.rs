#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/<name>")]
fn name(name: String) -> String {
    format!("hello {}",name)
}