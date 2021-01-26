use rocket::*;
use rocket_contrib::json::Json;
use serde_json::*;
use serde_json::Value;

use super::db::Conn as DbConn;
use super::models::{NewTodo, Todo};

#[get("/todos", format = "application/json")]
pub fn get_all_todo(conn: DbConn) -> Json<Value> {
    let users = Todo::get_all_todos(&conn);
    Json(json!({
        "status": 200,
        "result": users,
    }))
}

#[post("/todos", format = "application/json", data = "<new_todo>")]
pub fn new_todo(conn: DbConn, new_todo: Json<NewTodo>) -> Json<Value> {
    Json(json!({
        "status": Todo::insert_todo(new_todo.into_inner(), &conn),
        "result": Todo::get_all_todos(&conn).first(),
    }))
}

#[get("/todos/<id>")]
pub fn get_todo_by_id(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": 200,
        "result": Todo::get_todo_by_id(id, &conn),
    }))
}

#[put("/todos/<id>")]
pub fn mark_todo_as_done(conn: DbConn, id: i32) -> Json<Value> {
    Json(json!({
        "status": Todo::mark_as_done(id, &conn),
        "result": Todo::get_todo_by_id(id, &conn).first(),
    }))
}
