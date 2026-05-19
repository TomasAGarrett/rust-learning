use std::fs;
use std::io;
use std::io::Write; // Import Write trait for flushing stdout
use serde::{Serialize, Deserialize}; // Import Serialize and Deserialize traits from serde

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    description: String,
    done: bool,
}
fn read_input(prompt: &str) -> String { // Function to read user input with a prompt
    print!("{}", prompt); // Print the prompt without a newline
    io::stdout().flush().unwrap(); // Flush the output to ensure the prompt is displayed before reading input
    let mut input = String::new(); // Create a mutable String to store the input
    io::stdin().read_line(&mut input).unwrap(); // Read a line of input 
    input.trim().to_string() // Trim whitespace and convert to String
}

fn save_tasks(tasks: &Vec<Task>) { // Function to save tasks to a JSON file
    let json = serde_json::to_string(tasks).unwrap(); // Serialize the tasks vector to a JSON string
    fs::write("tasks.json", json).unwrap(); // Write the JSON string to a file named "tasks.json"
}

fn load_tasks() -> Vec<Task> { // Function to load tasks from a JSON file
    let file = fs::read_to_string("tasks.json"); // Try to read the contents of "tasks.json"
    match file { // Match the result of reading the file
        Ok(json) => // If the file is read successfully, deserialize the JSON string into a vector of tasks
serde_json::from_str(&json).unwrap(), // If the file is read successfully, deserialize the JSON string into a vector of tasks
        Err(_) => Vec::new(), // If there is an error (e.g., file not found), return an empty vector
    }
}

fn main() { // Main function to run the task manager
    let mut tasks: Vec<Task> = load_tasks(); // Load existing tasks from the JSON file into a mutable vector

    loop { // Start an infinite loop to continuously prompt the user for commands
        let input = read_input("\ncommand (add, list, done, delete, quit): "); // Read a command from the user

        match input.as_str() { // Match the input command
            "quit" => break,

            "add" => {
                let description = read_input("Enter task description: "); // Read task description from user
                if description.is_empty() {
                    println!("Description cannot be empty."); // Check if the description is empty
                    continue; // If empty, skip to the next iteration of the loop
                }
                tasks.push(Task {  // Add a new task to the tasks vector
                    description,
                    done: false,
                });
                save_tasks(&tasks); // Save the updated tasks to the JSON file
                println!("Task added."); // Confirm that the task has been added
            }
            "list" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                    continue; // If there are no tasks, skip to the next iteration of the loop
                }
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.done { "done"} else { "not done" }; // Determine the status of the task
                    println!("{}: {} - {}", i, task.description, status); // Print the task with its index and status
                }
            }

            "done" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                    continue; // If there are no tasks, skip to the next iteration of the loop
                }
                let index_str = read_input("Enter task number to mark as done: "); // Read the task number from the user
                match index_str.parse::<usize>() { // Try to parse the input as a usize
                    Ok(index) if index < tasks.len() => { // Check if the index is valid
                        tasks[index].done = true; // Mark the task as done
                        save_tasks(&tasks); // Save the updated tasks to the JSON file
                        println!("Task marked as done."); // Mark the task as done and confirm
                    }
                    _ => println!("Invalid task number."), // Handle invalid task numbers
                }
            }

            "delete" => {
                if tasks.is_empty() {
                    println!("No tasks available.");
                    continue; // If there are no tasks, skip to the next iteration of the loop
                }
                let index_str = read_input("Enter task number to delete: "); // Read the task number from the user
                match index_str.parse::<usize>() { // Try to parse the input as a usize
                    Ok(index) if index < tasks.len() => { // Check if the index is valid
                        let removed = tasks.remove(index); // Remove the task from the vector
                        save_tasks(&tasks); // Save the updated tasks to the JSON file
                        println!("Task '{}' deleted.", removed.description); // Confirm that the task has been deleted
                    }
                    _ => println!("Invalid task number."), // Handle invalid task numbers
                }
            }
            _ => println!("Unknown command"), // Handle unknown commands
        }
    }
}
/* 
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
*/