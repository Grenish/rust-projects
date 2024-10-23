# ToDo List - Code Explanation

This Rust program implements a simple command-line ToDo list manager. It allows users to add, remove, and mark tasks as complete, while saving and loading tasks from a file (`tasks.txt`). Below is a detailed explanation of how the code works, including the use of key components and terms.

## Components of the Code

### 1. `use std::fs; use std::fs::OpenOptions; use std::io::{self, Write};`

- **`use` statements**: These import standard library modules used in the program.
  - **`std::fs`**: The file system module, which provides file handling capabilities (read, write, open, etc.).
  - **`std::io::{self, Write}`**: The I/O module provides input/output handling, and the `Write` trait is for writing output to a file or other streams.
  - **`OpenOptions`**: This struct is used to configure how a file is opened, allowing options like writing, truncating, or creating a file.

### 2. Struct Definition: `Task`

```rust
struct Task {
    description: String,
    is_done: bool,
}
```

- **`struct Task`**: Defines a **struct** named `Task` to represent a single task in the ToDo list.
  - **`description: String`**: A field that stores the task description as a `String`.
  - **`is_done: bool`**: A boolean flag that represents whether the task is completed or not.

### 3. Task Implementation: `impl Task`

The `impl` block defines methods (or functions) associated with the `Task` struct:

```rust
impl Task {
    fn new(description: String) -> Task {
        Task {
            description,
            is_done: false,
        }
    }

    fn display(&self) -> String {
        if self.is_done {
            format!("[x] {}", self.description)
        } else {
            format!("[ ] {}", self.description)
        }
    }
}
```

- **`fn new(description: String) -> Task`**:

  - **`new`** is a constructor function that creates a new `Task` object with a provided description.
  - It initializes the `is_done` field to `false` because the task is incomplete by default.
  - **`-> Task`**: This indicates that the function returns a `Task` instance.

- **`fn display(&self) -> String`**:
  - This method returns a formatted string representation of the task, indicating whether it's complete (`[x]`) or incomplete (`[ ]`).
  - **`&self`**: A reference to the task object, required in method definitions to access the instance's fields.

### 4. Main Function: `fn main()`

```rust
fn main() {
    let mut tasks: Vec<Task> = load_tasks_from_file();
```

- **`let mut tasks: Vec<Task>`**:
  - **`mut`**: The `mut` keyword denotes that `tasks` is mutable, meaning its contents can change (tasks can be added or removed).
  - **`Vec<Task>`**: A vector (resizable array) that holds `Task` objects, representing the ToDo list.
  - **`load_tasks_from_file()`**: Loads existing tasks from the `tasks.txt` file or returns an empty list if the file doesn't exist.

### 5. User Interaction Loop: `loop`

```rust
    loop {
        println!("\n--ToDo List--");
        for (i, task) in tasks.iter().enumerate() {
            println!("{}: {}", i + 1, task.display());
        }
```

- **`loop {}`**: An infinite loop that keeps running until the user chooses to exit.
  - **`for (i, task) in tasks.iter().enumerate()`**: Loops through the `tasks` vector, displaying each task with an index.
    - **`iter()`**: Creates an iterator over the tasks vector.
    - **`enumerate()`**: Adds an index to each element (starting from 0).
    - **`task.display()`**: Calls the `display` method of the `Task` struct to show the task status.

### 6. Menu Display and User Input

```rust
        println!("\nEnter a command:");
        println!("1. Add a task");
        println!("2. Remove a task");
        println!("3. Mark a task as complete");
        println!("4. Quit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
```

- The program displays the available options for user input.
- **`let mut input = String::new()`**: Creates a mutable `String` to store the user's input.
  - **`String::new()`**: Initializes an empty string.
  - **`read_line(&mut input)`**: Reads user input from standard input (the console) and stores it in `input`. The `mut` keyword indicates the string can be modified.

### 7. Command Handling: `match input.trim()`

```rust
        match input.trim() {
            "1" => {
                println!("Enter task description: ");
                let mut task_desc = String::new();
                io::stdin().read_line(&mut task_desc).expect("Failed to read.");
                tasks.push(Task::new(task_desc.trim().to_string()));
                save_tasks_to_file(&tasks);
            }
```

- **`match`**: A control flow construct used to handle different user input.
  - **`"1" =>`**: If the user inputs `1`, they are prompted to enter a task description. The new task is created using `Task::new()` and added to the `tasks` vector with **`push()`**.
  - **`save_tasks_to_file(&tasks)`**: After adding a task, the list is saved to the file.

The other cases (`"2"`, `"3"`, `"4"`) follow similar patterns:

- **Removing a task**: The user enters the task number, and it's removed from the list.
- **Marking a task as complete**: The task’s `is_done` field is set to `true`.
- **Exiting**: `"4"` breaks the loop, terminating the program.

### 8. File Handling: `save_tasks_to_file()` and `load_tasks_from_file()`

These functions are responsible for saving and loading the task list to/from a file.

```rust
fn save_tasks_to_file(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.txt")
        .expect("Failed to open file.");
```

- **`save_tasks_to_file()`**:
  - This function saves the current task list to a file, `tasks.txt`.
  - **`OpenOptions::new().write(true).truncate(true).create(true)`**: Opens the file for writing, truncates (overwrites) it if it exists, or creates it if it doesn't.
  - The `for` loop writes each task description and its status (`1` for done, `0` for not done) to the file.

```rust
fn load_tasks_from_file() -> Vec<Task> {
    let contents = fs::read_to_string("tasks.txt").unwrap_or_default();
```

- **`load_tasks_from_file()`**:
  - Reads the `tasks.txt` file contents and returns a vector of `Task` objects. If the file doesn't exist, it returns an empty vector (`unwrap_or_default()`).
  - Each line in the file is split at the comma (`split(',')`) to separate the description and status.

## Important Keywords

### **`mut`**

- The **`mut`** keyword makes a variable mutable, allowing it to be modified after its initial assignment. In this program, variables like `tasks`, `input`, and `task_desc` are mutable because their values change during execution.

### **`self` & `&self`**

- **`self`**: Refers to the instance of the struct within a method. It allows accessing the fields of the struct.
- **`&self`**: Borrowing the instance as an immutable reference, meaning the method can read but not modify the struct fields.

### **`Vec<T>`**

- **`Vec<Task>`**: A vector is a dynamic array that can grow or shrink. In this program, the tasks are stored in a vector because the number of tasks is variable.

## Summary

This ToDo list application is a simple yet effective demonstration of Rust’s strengths in handling file I/O, user interaction, and working with mutable data. It uses structs, loops, and file operations to create a persistent task list that users can manage through a command-line interface. Each part of the code serves to either manipulate the task list or interact with the user in a clean and efficient manner.
