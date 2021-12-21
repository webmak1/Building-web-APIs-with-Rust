
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
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Json<models::Rustacean> {
    conn.run(move |c| {
        let rustacean = rustaceans::table.find(id)
            .get_result::<Rustacean>(c)
            .expect("Error loading rustacean from DB");
    Json(rustacean)
    }).await
}

#[post("/rustaceans", format = "json", data="<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<NewRustacean>) -> Json<usize>  {
    conn.run(|c| {
    let result = diesel::insert_into(rustaceans::table)
            .values(new_rustacean.into_inner())
            .execute(c).expect("Error adding restaceans to DB");
    Json(result)
    }).await
}

#[put("/rustaceans/<id>", format = "json", data="<rustacean>")]
async fn update_rustacean(id: i32, _auth: BasicAuth, conn: DbConn, rustacean: Json<Rustacean>) -> Json<usize>  {
    conn.run(move |c| {
    let result = diesel::update(rustaceans::table.find(id))
        .set(
            (
            rustaceans::name.eq(rustacean.name.to_owned()),
            rustaceans::email.eq(rustacean.email.to_owned()),
            ))
        .execute(c)
        .expect("Error updating rustaceans to DB");
    Json(result)
    }).await
}

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> status::NoContent {
    conn.run(move |c| {
    diesel::delete(rustaceans::table.find(id))
        .execute(c)
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
