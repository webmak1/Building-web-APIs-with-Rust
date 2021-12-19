
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod auth;
mod models;
mod schema;

use auth::BasicAuth;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
// use rocket_sync_db_pools::{database, diesel};
use rocket_sync_db_pools::{database};

use diesel::prelude::*;
use models::*;
use schema::*;


#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
}

#[get("/rustaceans", format = "json")]
async fn get_rustaceans(_auth: BasicAuth, conn: DbConn) -> Json<Vec<models::Rustacean>> {
    conn.run(|c| {
        let all = rustaceans::table.limit(100).load::<Rustacean>(c).expect("Error loading rustaceans from DB!");
        Json(all)
    }).await
}

#[get("/rustaceans/<id>")]
fn view_rustacean(id: i32, _auth: BasicAuth) -> Json<User> {
    let person = User {id: 1, name: "John Doe".to_owned() };
    Json(person)
}

#[post("/rustaceans", format = "json")]
fn create_rustacean(_auth: BasicAuth) -> Json<User>  {
    let person = User {id: 1, name: "John Doe".to_owned() };
    Json(person)
}

#[put("/rustaceans/<id>", format = "json")]
fn update_rustacean(id: i32, _auth: BasicAuth) -> Json<User>  {
    let person = User {id: 1, name: "John Doe".to_owned() };
    Json(person)
}

#[delete("/rustaceans/<_id>")]
fn delete_rustacean(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("Not found!")
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
      .register("/", catchers![
                not_found
      ])
      .attach(DbConn::fairing())
      .launch()
      .await;
}
