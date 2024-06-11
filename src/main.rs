#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world, Hey!"
}

// #[rocket::main]
// async fn main() {
//     let _ = rocket::build()
//         .mount("/", routes![hello])
//         .launch()
//         .await;
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
}