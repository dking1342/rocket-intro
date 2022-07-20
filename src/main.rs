#[macro_use] extern crate rocket;
use rocket_contrib::serve::StaticFiles;

// #[get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }

// #[get("/<name>")]
// fn name(name: String) -> String {
//     format!("hello {}",name)
// }

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![StaticFiles::from("/routes/index")])
}