mod database;
mod todo;

use crate::todo::TodoList;
use anyhow::{Ok, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Get datastore and session
    let db = &(database::get_datastore_session().await?);

    let mut todo = TodoList::new();

    // --- Create
    let task_id = todo.add_task(db, "Buy Milk", 5).await?;
    let task_id_2 = todo.add_task(db, "Buy Milk", 5).await?;
    println!("The task id after create is: {task_id}");
    println!("The task id after create is: {task_id_2}");

    // --- Merge/Update
    let task_id = todo
        .merge_task(
            db,
            &task_id,
            "Get milk from store",
            todo::Status::NotStarted,
            06,
        )
        .await?;
    println!("merged task id is {task_id}");

    // -- delete task
    // let task_id = database::delete_task(db, t1).await?;
    // println!("deleted task id is {task_id}");

    // -- get all tasks
    let all_tasks = todo.get_all_tasks(db).await?;
    println!("The returned tasks is {:?}", all_tasks);

    Ok(())
}
