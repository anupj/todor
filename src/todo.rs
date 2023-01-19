use core::str;

use crate::database;
use crate::database::DB;
use anyhow::{Ok, Result};

#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<Task>,
}

#[derive(Debug, Clone)]
pub(crate) struct Task {
    id: String,
    title: String,
    status: Status,
    priority: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    NotStarted,
    Completed,
    Archived, // soft-deleted
}

impl From<Status> for &str {
    fn from(value: Status) -> Self {
        match value {
            Status::NotStarted => "Not Started",
            Status::Completed => "Completed",
            Status::Archived => "Archived",
        }
    }
}

impl From<String> for Status {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Not Started" => Status::NotStarted,
            "\"Not Started\"" => Status::NotStarted,
            "Completed" => Status::Completed,
            "\"Completed\"" => Status::Completed,
            "\"Archived\"" => Status::Archived,
            "Archived" => Status::Archived,
            &_ => Status::NotStarted,
        }
    }
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::<Task>::new(),
        }
    }

    // add a new task to the db
    pub async fn add_task(&mut self, db: &DB, title: &str, priority: u8) -> Result<String> {
        let task_id = database::create_task(db, title, priority, Status::NotStarted.into()).await?;
        Ok(task_id)
    }

    // archive_task
    pub async fn archive_task<'a>(&mut self, db: &DB, task_id: &'a str) -> Result<&'a str> {
        let task_id = database::archive_task(db, task_id).await?;
        Ok(task_id)
    }

    // delete task
    pub async fn delete_task<'a>(&mut self, db: &DB, task_id: &'a str) -> Result<&'a str> {
        let task_id = database::hard_delete_task(db, task_id).await?;
        Ok(task_id)
    }

    // update an existing task
    pub async fn update_task<'a>(
        &mut self,
        db: &DB,
        task_id: &'a str,
        title: &'a str,
        status: Status,
        priority: u8,
    ) -> Result<&'a str> {
        let task_id = database::update_task(db, task_id, title, status.into(), priority).await?;
        Ok(task_id)
    }

    // Return the `TodoList` struct
    pub async fn get_all_tasks(&self, db: &DB) -> Result<TodoList> {
        let tasks = database::get_all_tasks(db).await?;
        let mut todo_list = TodoList::new();

        for task in tasks {
            todo_list.tasks.push(Task {
                id: task.0,
                title: task.1,
                status: task.2.into(),
                priority: task.3,
            })
        }
        Ok(todo_list)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn test_add_tasks_to_database() -> Result<()> {
        let db = &(database::get_datastore_session().await?);

        let mut todo = TodoList::new();

        // -- create tasks
        let task_id = todo.add_task(db, "Buy Milk", 5).await?;
        let task_id_2 = todo.add_task(db, "Finish project", 1).await?;

        // --get all tasks
        let all_tasks = todo.get_all_tasks(db).await?;

        // Assert
        assert!(task_id.is_empty() != true);
        assert!(task_id_2.is_empty() != true);
        assert!(all_tasks.tasks.is_empty() != true);

        for task in all_tasks.tasks.iter() {
            match task.title.as_str() {
                "Buy Milk" => assert_eq!(task.priority, 5),
                "Finish project" => assert_eq!(task.priority, 1),
                _ => assert!(task.title.is_empty() != true),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_update_tasks() -> Result<()> {
        let db = &(database::get_datastore_session().await?);

        let mut todo = TodoList::new();

        // -- create tasks
        let task_id = todo.add_task(db, "Buy Milk", 5).await?;
        let task_id_2 = todo.add_task(db, "Finish project", 1).await?;

        // --- Merge/Update
        let task_id = todo
            .update_task(db, &task_id, "Get milk from store", Status::Completed, 06)
            .await?;

        // --get all tasks
        let all_tasks = todo.get_all_tasks(db).await?;

        // Assert
        assert!(task_id.is_empty() != true);
        assert!(task_id_2.is_empty() != true);
        assert!(all_tasks.tasks.is_empty() != true);

        for task in all_tasks.tasks.iter() {
            match task.title.as_str() {
                "Get milk from store" => assert_eq!(task.status, Status::Completed),
                _ => assert!(task.title.is_empty() != true),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_archive_task() -> Result<()> {
        let db = &(database::get_datastore_session().await?);

        let mut todo = TodoList::new();

        // -- create tasks
        let task_id = todo.add_task(db, "Buy Milk", 5).await?;
        let task_id_2 = todo.add_task(db, "Finish project", 1).await?;


        // -- archive task id 1
        let task_id = todo.archive_task(db, task_id.as_str()).await?;
        println!("Archived task id 1 {task_id}");

        // --get all tasks
        let all_tasks = todo.get_all_tasks(db).await?;

        // Assert
        assert!(task_id.is_empty() != true);
        assert!(task_id_2.is_empty() != true);
        assert!(all_tasks.tasks.is_empty() != true);

        for task in all_tasks.tasks.iter() {
            match task.title.as_str() {
                "Buy Milk" => assert_eq!(task.status, Status::Archived),
                _ => assert!(task.title.is_empty() != true),
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_delete_task() -> Result<()> {
        let db = &(database::get_datastore_session().await?);

        let mut todo = TodoList::new();

        // -- create tasks
        let task_id = todo.add_task(db, "Buy Milk", 5).await?;
        let task_id_2 = todo.add_task(db, "Finish project", 1).await?;

        // -- delete task id 1
        let task_id = todo.delete_task(db, task_id.as_str()).await?;
        let task_id_2 = todo.delete_task(db, task_id_2.as_str()).await?;

        // --get all tasks
        let all_tasks = todo.get_all_tasks(db).await?;

        // Assert
        assert!(task_id.is_empty() != true);
        assert!(task_id_2.is_empty() != true);

        // there should be no tasks in the db table
        assert!(all_tasks.tasks.is_empty() == true);

        Ok(())
    }
}
