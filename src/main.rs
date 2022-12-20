use clap::{Parser, Subcommand};

use tasks::{TaskStack};

mod tasks;

const TASKS_FILE: &str = "tasks.txt";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// add a task
    Add { content: Vec<String> },

    /// list all tasks
    List,

    /// mark a task as complete
    Complete { number: usize },

    /// remove a task
    Remove { number: usize },
}

fn main() {
    // todo, better error handling for incorrectly formatted task list
    let mut tasks = match TaskStack::from_file(TASKS_FILE) {
        Ok(tasks) => tasks,
        Err(_) => TaskStack::new(),
    };

    let args = Cli::parse();
    match args.command {
        Commands::Add { content } => tasks.add(content.join(" ")),
        Commands::List => handle_list_tasks(),
        Commands::Complete { number } => {
            match tasks.complete(number) {
                Ok(_) => (),
                Err(e) => println!("unable to mark task as completed: {}", e),
            };
        }
        Commands::Remove { number } => {
            match tasks.remove(number) {
                Ok(_) => (),
                Err(e) => println!("unable to remove task: {}", e),
            };
        }
    }
    match tasks.write_to_file(TASKS_FILE) {
        Ok(_) => (),
        Err(e) => println!("unable to save changes: {}", e),
    };
}

fn handle_list_tasks() {}
