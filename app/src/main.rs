
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod auth;
mod models;
mod schema;
mod repositories;

use auth::BasicAuth;
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::serde::{Serialize, Deserialize};
// use rocket_sync_db_pools::{database, diesel};
use rocket_sync_db_pools::{database};

use models::*;
use repositories::*;


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
        let all = RustaceanRepository::load_all(c)
            .expect("Error loading rustaceans from DB!");
        Json(all)
    }).await
}

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Json<models::Rustacean> {
    conn.run(move |c| {
        let rustacean = RustaceanRepository::find(c, id) 
            .expect("Error loading rustacean from DB");
    Json(rustacean)
    }).await
}

#[post("/rustaceans", format = "json", data="<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<NewRustacean>) -> Json<models::Rustacean>  {
    conn.run(|c| {
    let result = RustaceanRepository::create(c, new_rustacean.into_inner()) 
            .expect("Error adding restaceans to DB");
    Json(result)
    }).await
}

#[put("/rustaceans/<_id>", format = "json", data="<rustacean>")]
async fn update_rustacean(_id: i32, _auth: BasicAuth, conn: DbConn, rustacean: Json<Rustacean>) -> Json<models::Rustacean>  {
    conn.run(move |c| {
    let result = RustaceanRepository::save(c, rustacean.into_inner()) 
        .expect("Error updating rustaceans to DB");
    Json(result)
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> status::NoContent {
    conn.run(move |c| {
    RustaceanRepository::delete(c, id)
        .expect("Error deleting rustaceans to DB");
    status::NoContent
    }).await
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
