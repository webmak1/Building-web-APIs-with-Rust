#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_migrations;

mod auth;
mod models;
mod schema;
mod repositories;

use auth::BasicAuth;
use rocket::response::status;
use rocket::http::Status;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket_sync_db_pools::{database};

use models::*;
use repositories::*;

embed_migrations!();

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

// #[get("/rustaceans", format = "json")]
// async fn get_rustaceans(_auth: BasicAuth, conn: DbConn) -> Json<Vec<models::Rustacean>> {
//     conn.run(|c| {
//         let all = RustaceanRepository::load_all(c)
//             .expect("Error loading rustaceans from DB!");
//         Json(all)
//     }).await
// }

#[get("/rustaceans", format = "json")]
async fn get_rustaceans(_auth: BasicAuth, conn: DbConn) -> Result<Json<Vec<models::Rustacean>>, status::Custom<Json<std::string::String>>> {
    conn.run(|c| {
        RustaceanRepository::load_all(c)
            .map(| rustaceans | Json(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))
    }).await
}

// #[get("/rustaceans/<id>")]
// async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Json<models::Rustacean> {
//     conn.run(move |c| {
//         let rustacean = RustaceanRepository::find(c, id) 
//             .expect("Error loading rustacean from DB");
//     Json(rustacean)
//     }).await
// }

#[get("/rustaceans/<id>")]
async fn view_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Result<Json<models::Rustacean>, status::Custom<Json<std::string::String>>> {
    conn.run(move |c| {
        RustaceanRepository::find(c, id) 
            .map(| rustaceans | Json(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))
    }).await
}

// #[post("/rustaceans", format = "json", data="<new_rustacean>")]
// async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<NewRustacean>) -> Json<models::Rustacean>  {
//     conn.run(|c| {
//     let result = RustaceanRepository::create(c, new_rustacean.into_inner()) 
//             .expect("Error adding restaceans to DB");
//     Json(result)
//     }).await
// }

#[post("/rustaceans", format = "json", data="<new_rustacean>")]
async fn create_rustacean(_auth: BasicAuth, conn: DbConn, new_rustacean: Json<NewRustacean>) -> Result<Json<models::Rustacean>, status::Custom<Json<std::string::String>>> {
    conn.run(|c| {
    RustaceanRepository::create(c, new_rustacean.into_inner()) 
    .map(| rustaceans | Json(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))
    }).await
}


// #[put("/rustaceans/<_id>", format = "json", data="<rustacean>")]
// async fn update_rustacean(_id: i32, _auth: BasicAuth, conn: DbConn, rustacean: Json<Rustacean>) -> Json<models::Rustacean>  {
//     conn.run(move |c| {
//     let result = RustaceanRepository::save(c, rustacean.into_inner()) 
//         .expect("Error updating rustaceans to DB");
//     Json(result)
//     }).await
// }

#[put("/rustaceans/<_id>", format = "json", data="<rustacean>")]
async fn update_rustacean(_id: i32, _auth: BasicAuth, conn: DbConn, rustacean: Json<Rustacean>) -> Result<Json<models::Rustacean>, status::Custom<Json<std::string::String>>> {
    conn.run(move |c| {
    RustaceanRepository::save(c, rustacean.into_inner()) 
    .map(| rustaceans | Json(rustaceans))
            .map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))
    }).await
}

// #[delete("/rustaceans/<id>")]
// async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> status::NoContent {
//     conn.run(move |c| {
//     RustaceanRepository::delete(c, id)
//         .expect("Error deleting rustaceans to DB");
//     status::NoContent
//     }).await
// }

#[delete("/rustaceans/<id>")]
async fn delete_rustacean(id: i32, _auth: BasicAuth, conn: DbConn) -> Result<status::NoContent, status::Custom<Json<std::string::String>>> {
    conn.run(move |c| {
    RustaceanRepository::delete(c, id)
    .map(| _ | status::NoContent)
            .map_err(|e| status::Custom(Status::InternalServerError, Json(e.to_string())))
    }).await
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("Not found!")
}

async fn run_db_migrations(rocket: rocket::Rocket<rocket::Build>) -> Result<rocket::Rocket<rocket::Build>, rocket::Rocket<rocket::Build>> {
    DbConn::get_one(&rocket).await
        .expect("failed to retrieve database connection")
        .run(|c| match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(e) => {
                println!("Failed to run database migrations: {:?}", e);
                Err(rocket)
            }
        }).await
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
      .attach(AdHoc::try_on_ignite("Database Migrations", run_db_migrations))
      .launch()
      .await;
}
