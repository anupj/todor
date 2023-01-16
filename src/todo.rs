pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::<Task>::new(),
        }
    }

    // add a new task to the
    // todo list
    pub fn add_task(&mut self, t: String) {
        self.tasks.push(Task(t, Status::NotStarted));
    }

    // Returns a tuple of task name and task status
    pub fn get_tasks(&self) -> Vec<(String, Status)> {
        // mapping `Task` to a `String`
        // and returning `Vec<(String, Status)>`
        // I'm cloning the `String` to
        // avoid "moving" it out of this fn
        self.tasks.iter().map(|t| (t.0.clone(), t.1)).collect()
    }
}

#[derive(Debug, Clone)]
struct Task(String, Status);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    NotStarted,
    Completed,
    Archived, // soft-deleted
}

#[cfg(test)]
mod test {
    use super::Status;
    use super::TodoList;

    #[test]
    fn add_tasks_to_my_todo_list_works() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Task 1".to_string());
        assert_eq!("Task 1".to_string(), todo_list.get_tasks()[0].0);
    }

    #[test]
    fn count_tasks_added_to_my_todo_list() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Task 1".to_string());
        todo_list.add_task("Task 2".to_string());
        todo_list.add_task("Task 3".to_string());
        todo_list.add_task("Task 4".to_string());
        todo_list.add_task("Task 5".to_string());
        assert_eq!(5, todo_list.get_tasks().len());
    }

    #[test]
    fn new_task_is_in_not_started_state() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Task 1".to_string());
        assert_eq!(Status::NotStarted, todo_list.get_tasks()[0].1);
    }
}
