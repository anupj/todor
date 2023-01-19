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
    println!("The task id of the first task after create is: {task_id}");
    println!("The task id of the second task after create is: {task_id_2}");

    // --- Merge/Update
    let task_id = todo
        .update_task(
            db,
            &task_id,
            "Get milk from store",
            todo::Status::NotStarted,
            06,
        )
        .await?;
    println!("Merged task id is {task_id}");

    // -- archive task id 1
    let task_id = todo.archive_task(db, task_id).await?;
    println!("Archived task id 1 {task_id}");

    // -- get all tasks
    let all_tasks = todo.get_all_tasks(db).await?;
    println!("The returned tasks are {:?}", all_tasks);

    Ok(())
}
