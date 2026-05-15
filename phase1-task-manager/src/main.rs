use std::io;
#[derive(Debug)]
struct Task {
    description: String,
    done: bool,
}

fn main() {
    let mut tasks:Vec<Task> = Vec::new();

    loop {
        println!("command (add, list, quit):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input= input.trim();

        if input == "quit" {
            break;
        } else if input == "add" {
            println!("Enter task description:");
            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            let task = Task {
                description: description.trim().to_string(),
                done: false,
            };
            tasks.push(task);
        } else if input == "list" {
            for(i, task) in tasks.iter().enumerate() {
                println!("{}: {} - done: {}", i, task.description, task.done);
            }
        } else if input == "done" {
            println!("Enter task number to mark as done:");
            let mut index_input = String::new();
            io::stdin().read_line(&mut index_input).unwrap();
            let index: usize = index_input.trim().parse().unwrap();
            tasks[index].done = true;
            println!("Task marked as done.");
        } else if input == "delete" {
            println!("Enter task number to delete:");
            let mut index_input = String::new();
            io::stdin().read_line(&mut index_input).unwrap();
            let index: usize = index_input.trim().parse().unwrap();
            tasks.remove(index);
            println!("Task deleted.");
        } else {
            println!("Unknown command");
        }
    }
}
