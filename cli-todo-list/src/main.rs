#[derive(Debug)]
struct Todo {
    id: i32,
    title: String,
    completed: bool,
    description: String,
}

#[derive(Debug)]
struct TodoList {
    todos: Vec<Todo>,
}

fn main() {
    let todoList = TodoList {
        todos: vec![
            Todo {
                id: 1,
                title: String::from("Market Expenses"),
                completed: false,
                description: String::from("Buy groceries"),
            },
            Todo {
                id: 2,
                title: String::from("Weather Expenses"),
                completed: false,
                description: String::from("Pay the gym"),
            },
            
        ],
    };    

    println!("user info: {:#?}", todoList);
}
