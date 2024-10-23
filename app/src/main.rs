use std::fs;
use std::fs::OpenOptions;
use std::io::{self, Write};

/// Represents a task in the ToDo list
struct Task {
    description: String,
    is_done: bool,
}

impl Task {
    /// Creates a new task with the given description
    fn new(description: String) -> Task {
        Task {
            description,
            is_done: false,
        }
    }

    /// Returns a formatted string representation of the task
    fn display(&self) -> String {
        if self.is_done {
            format!("[x] {}", self.description)
        } else {
            format!("[ ] {}", self.description)
        }
    }
}

fn main() {
    // Load tasks from file or create an empty vector if file doesn't exist
    let mut tasks: Vec<Task> = load_tasks_from_file();

    loop {
        // Display the current list of tasks
        println!("\n--ToDo List--");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task.display());
        }

        // Display the menu options
        println!("\nEnter a command:");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. Mark a task as complete");
        println!("4. Quit");

        // Read user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Process user input
        match input.trim() {
            "1" => {
                // Add a new task
                println!("Enter task description: ");
                let mut task_desc = String::new();
                io::stdin()
                    .read_line(&mut task_desc)
                    .expect("Failed to read.");
                tasks.push(Task::new(task_desc.trim().to_string()));
                save_tasks_to_file(&tasks);
            }
            "2" => {
                // Remove a task
                println!("Enter the task to remove: ");
                let mut task_num = String::new();
                io::stdin()
                    .read_line(&mut task_num)
                    .expect("Failed to read.");
                let task_num: usize = task_num.trim().parse().expect("Enter a valid number.");
                if task_num > 0 && task_num <= tasks.len() {
                    tasks.remove(task_num - 1);
                    save_tasks_to_file(&tasks);
                } else {
                    println!("\nInvalid task number.");
                }
            }
            "3" => {
                // Mark a task as complete
                println!("Enter the task to mark as complete: ");
                let mut task_num = String::new();
                io::stdin()
                    .read_line(&mut task_num)
                    .expect("Failed to read.");
                let task_num: usize = task_num.trim().parse().expect("Enter a valid number.");
                if task_num > 0 && task_num <= tasks.len() {
                    tasks[task_num - 1].is_done = true;
                    save_tasks_to_file(&tasks);
                } else {
                    println!("\nInvalid task number.");
                }
            }
            "4" => break, // Exit the program
            _ => println!("Invalid command."),
        }
    }
}

/// Saves the current list of tasks to a file
fn save_tasks_to_file(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.txt")
        .expect("Failed to open file.");

    for task in tasks {
        let status = if task.is_done { "1" } else { "0" };
        writeln!(file, "{},{}", task.description, status).expect("Unable to write file.");
    }
}

/// Loads tasks from a file, returning an empty vector if the file doesn't exist
fn load_tasks_from_file() -> Vec<Task> {
    let contents = fs::read_to_string("tasks.txt").unwrap_or_default();
    contents
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').collect();
            let description = parts[0].to_string();
            let is_done = parts.get(1).map_or(false, |&status| status == "1");
            Task {
                description,
                is_done,
            }
        })
        .collect()
}