use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[get("/rustaceans", format = "json")]
fn get_rustaceans() -> Json<User> {
    let person = User {id: 1, name: "John Doe".to_owned() };
    Json(person)
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32) -> Json<User> {
    let person = User {id: 1, name: "John Doe".to_owned() };
    Json(person)
}

#[post("/rustaceans", format = "json")]
fn create_rustacean() -> Json<User>  {
    let person = User {id: 1, name: "John Doe".to_owned() };
    Json(person)
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32) -> Json<User>  {
    let person = User {id: 1, name: "John Doe".to_owned() };
    Json(person)
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32) -> status::NoContent {
    status::NoContent
}

#[rocket::main]
async fn main() {
     let _ = rocket::build()
      .mount("/", routes![
            get_rustaceans,
            view_rustacean,
            create_rustacean,
            update_rustacean,
            delete_rustacean
        ])
      .launch()
      .await;
}
