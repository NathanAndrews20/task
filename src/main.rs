use clap::{Parser, Subcommand};
use ansi_term::Style;

use tasks::TaskStack;

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
    let mut task_stack = match TaskStack::from_file(TASKS_FILE) {
        Ok(ts) => ts,
        Err(_) => TaskStack::new(),
    };

    let args = Cli::parse();
    match args.command {
        Commands::Add {
            content: raw_content,
        } => {
            let content = raw_content.join(" ").trim().to_string();
            if !content.is_empty() {
                task_stack.add(content)
            } else {
                println!("cannot add empty task")
            }
        }

        Commands::List => {
            for (task_num, task) in task_stack.tasks().enumerate() {
                let task_content =  if task.completed {
                    let style = Style::new();
                    style.strikethrough().paint(task.content.clone()).to_string()
                } else {
                    task.content.clone()
                };
                println!("{}: {}", task_num + 1, task_content);
            }
        }
        Commands::Complete { number: task_number } => {
            if task_number < 1 {
                println!("unable to mark task as completed: no task with number 0");
                return
            }
            match task_stack.complete(task_number - 1) {
                Ok(_) => (),
                Err(e) => println!("unable to mark task as completed: {}", e),
            };
        }
        Commands::Remove { number: task_number } => {
            if task_number < 1 {
                println!("unable to mark task as completed: no task with number 0");
                return
            }
            match task_stack.remove(task_number - 1) {
                Ok(_) => (),
                Err(e) => println!("unable to remove task: {}", e),
            };
        }
    }
    match task_stack.write_to_file(TASKS_FILE) {
        Ok(_) => (),
        Err(e) => println!("unable to save changes: {}", e),
    };
}
