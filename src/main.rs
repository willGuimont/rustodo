#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::env;

use dotenv::dotenv;

use routes::*;

mod db;
mod models;
mod routes;
mod schema;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");

    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api",
            routes![get_all_todo, new_todo, get_todo_by_id, mark_todo_as_done],
        )
}

fn main() {
    rocket().launch();
}
//
// use rocket::Request;
//
// use self::diesel::prelude::*;
// use self::models::*;
// use self::rustodo::*;
//
// #[database("todos")]
// struct TodosDbConn(diesel::SqliteConnection);
//
// #[get("/todos")]
// fn get_todos(connection: TodosDbConn) -> &'static str {
//     use rustodo::schema::todos::dsl::*;
//
//     let todo = todos.filter(id.eq(1)).limit(5).get_results::<Todo>(&connection.0);
//     // let results = todos.filter(done.eq(true))
//     //     .limit(5)
//     //     .load::<Todo>(&connection)
//     //     .expect("Error loading posts");
//
//     // println!("{:?}", todo);
//     "Hello world"
// }
//
// #[catch(503)]
// fn service_not_available(_req: &Request) -> &'static str {
//     "Service is not available. (Is the database up?)"
// }
//
// fn main() {
//     rocket::ignite()
//         .attach(TodosDbConn::fairing())
//         .register(catchers![service_not_available])
//         .mount("/api", routes![get_todos])
//         .launch();
// }
