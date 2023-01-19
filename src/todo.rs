use core::str;

use crate::database;
use crate::database::DB;
use anyhow::{Ok, Result};

#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<Task>,
}

// Task(id, title, status, priority)
#[derive(Debug, Clone)]
pub(crate) struct Task(String, String, Status, u8);

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

    // Returns a tuple of task name and task status
    pub async fn get_all_tasks(&self, db: &DB) -> Result<TodoList> {
        let tasks = database::get_all_tasks(db).await?;
        let mut todo_list = TodoList::new();

        for task in tasks {
            todo_list
                .tasks
                .push(Task(task.0, task.1, task.2.into(), task.3))
        }
        Ok(todo_list)
    }
}
