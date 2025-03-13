use std::io;

#[derive(Debug)]
struct Task {
    id: i32,
    title: String,
    description: String,
    completed: bool,
}

impl Task {    
    fn new(id: i32, title: String, description: String, completed: bool) -> Task {
        Task { id: id, title: title, description: description, completed: completed }
    }

    fn print_task(self) {
        println!("{:#?}", self);        
    }
}

fn main() {    
    let task = Task::new(1, String::from("Buy Groceries"), String::from("Buy 2 apples and 1 banana"), false);    
    task.print_task();    
}
