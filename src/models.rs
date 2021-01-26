extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use diesel;
use diesel::prelude::*;
use diesel::SqliteConnection;

use super::schema::todos;
use super::schema::todos::dsl::todos as all_todos;

#[derive(Debug, Serialize, Queryable, Identifiable)]
pub struct Todo {
    pub id: i32,
    pub message: String,
    pub done: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
// #[derive(Serialize, Deserialize, <todos as Trait>::table)]
#[table_name = "todos"]
pub struct NewTodo {
    #[column_name = "message"]
    pub message: String,
}

impl Todo {
    pub fn get_all_todos(conn: &SqliteConnection) -> Vec<Todo> {
        all_todos
            .order(todos::id.desc())
            .load::<Todo>(conn)
            .expect("error loading all todos")
    }

    pub fn insert_todo(todo: NewTodo, conn: &SqliteConnection) -> bool {
        diesel::insert_into(todos::table)
            .values(&todo)
            .execute(conn)
            .is_ok()
    }

    pub fn get_todo_by_id(id: i32, conn: &SqliteConnection) -> Vec<Todo> {
        all_todos
            .filter(todos::id.eq(id))
            .load::<Todo>(conn)
            .expect("error get by id")
    }

    pub fn mark_as_done(id: i32, conn: &SqliteConnection) -> bool {
        diesel::update(todos::table)
            .filter(todos::id.eq(id))
            .set(todos::done.eq(true))
            .execute(conn)
            .is_ok()
    }
}
