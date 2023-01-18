#![allow(dead_code)]
#![allow(unused)]

mod database;
mod todo;

use crate::database::create_task;
use crate::database::DB;
use anyhow::{anyhow, Ok, Result};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Datetime, Object, Thing, Value};
use surrealdb::{Datastore, Response, Session};

#[tokio::main]
async fn main() -> Result<()> {
    // Get datastore and session
    let db = &(database::get_datastore_session().await?);

    // --- Create
    let t1 = create_task(db, "Task 01", 10, "NOT STARTED").await?;
    println!("{t1}");
    let t2 = create_task(db, "Task 02", 7, "IN PROGRESS").await?;
    println!("{t2}");

    // --- Merge/Update
    let task_id = database::update_task(db, t2, "COMPLETED").await?;
    println!("merged task id is {task_id}");

    // -- delete task
    let task_id = database::delete_task(db, t1).await?;
    println!("deleted task id is {task_id}");

    // -- get all tasks
    database::get_all_tasks(db).await?;

    Ok(())
}
