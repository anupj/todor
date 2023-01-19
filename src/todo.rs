use core::str;

use crate::database;
use crate::database::DB;
use anyhow::{Ok, Result};

pub struct TodoList {
    tasks: Vec<Task>,
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

    // merge or update an existing task
    pub async fn merge_task<'a>(
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
    pub async fn get_all_tasks(&self, db: &DB) -> Result<Vec<(String, Status, u8)>> {
        let tasks = database::get_all_tasks(db).await?;
        let mut response: Vec<(String, Status, u8)> = Vec::new();

        for task in tasks {
            response.push((task.0, task.1.as_str().into(), task.2));
        }
        Ok(response)
    }
}

// Task(title, status, priority)
#[derive(Debug, Clone)]
struct Task(String, Status, u8);

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

impl From<&str> for Status {
    fn from(value: &str) -> Self {
        match value {
            "Not Started" => Status::NotStarted,
            "Completed" => Status::Completed,
            "Archived" => Status::Archived,
            _ => Status::NotStarted,
        }
    }
}

#[cfg(test)]
mod test {
    //     use super::Status;
    //     use super::TodoList;
    //
    //     #[test]
    //     fn add_tasks_to_my_todo_list_works() {
    //         let mut todo_list = TodoList::new();
    //         todo_list.add_task("Task 1".to_string());
    //         assert_eq!("Task 1".to_string(), todo_list.get_tasks()[0].0);
    //     }
    //
    //     #[test]
    //     fn count_tasks_added_to_my_todo_list() {
    //         let mut todo_list = TodoList::new();
    //         todo_list.add_task("Task 1".to_string());
    //         todo_list.add_task("Task 2".to_string());
    //         todo_list.add_task("Task 3".to_string());
    //         todo_list.add_task("Task 4".to_string());
    //         todo_list.add_task("Task 5".to_string());
    //         assert_eq!(5, todo_list.get_tasks().len());
    //     }
    //
    //     #[test]
    //     fn new_task_is_in_not_started_state() {
    //         let mut todo_list = TodoList::new();
    //         todo_list.add_task("Task 1".to_string());
    //         assert_eq!(Status::NotStarted, todo_list.get_tasks()[0].1);
    //     }
}
