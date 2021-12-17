#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

#[get("/")]
fn hello() -> Json<&'static str> {
    Json("Hello, world!")
}

#[rocket::main]
async fn main() {
     let _ = rocket::build()
      .mount("/", routes![hello])
      .launch()
      .await;
}
