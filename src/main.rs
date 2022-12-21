use std::{thread, time::Duration};

use ansi_term::Style;
use clap::{ArgGroup, Parser, Subcommand};

use indicatif::{ProgressBar, ProgressStyle};
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
    List {
        #[arg(short = 'p', long = "progress")]
        progress: bool,
    },

    /// mark a task as complete
    Complete { number: usize },

    /// remove a task
    #[command(group(ArgGroup::new("remove").required(true).args(["number", "completed"])))]
    Remove {
        #[arg(short = 'n', long = "number")]
        number: Option<usize>,

        #[arg(short = 'c', long = "completed")]
        completed: bool,
    },
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

        Commands::List { progress } => {
            if progress {
                println!("total tasks: {}, completed tasks: {}, tasks remaining: {}", task_stack.num_tasks(), task_stack.num_tasks_completed(), task_stack.num_tasks() - task_stack.num_tasks_completed());
                let bar = ProgressBar::new(task_stack.num_tasks() as u64);                
                bar.set_style(ProgressStyle::with_template("[{wide_bar:.cyan/black}] {pos}/{len}").unwrap().progress_chars("#|-"));
                bar.inc(task_stack.num_tasks_completed() as u64);
                bar.abandon();
            } else {
                for (task_num, task) in task_stack.tasks().enumerate() {
                    let task_content = if task.completed {
                        let style = Style::new();
                        style
                            .strikethrough()
                            .paint(task.content.clone())
                            .to_string()
                    } else {
                        task.content.clone()
                    };
                    println!("{}: {}", task_num + 1, task_content);
                }
            }
        }
        Commands::Complete {
            number: task_number,
        } => {
            if task_number < 1 {
                println!("unable to mark task as completed: no task with number 0");
                return;
            }
            match task_stack.complete(task_number - 1) {
                Ok(_) => (),
                Err(e) => println!("unable to mark task as completed: {}", e),
            };
        }
        Commands::Remove {
            number: number_option,
            completed,
        } => match (number_option, completed) {
            (Some(task_number), _) => {
                if task_number < 1 {
                    println!("unable to mark task as completed: no task with number 0");
                    return;
                }
                match task_stack.remove(task_number - 1) {
                    Ok(_) => (),
                    Err(e) => println!("unable to remove task: {}", e),
                }
            }
            (_, true) => {
                if !task_stack.remove_completed() {
                    println!("unable to removed tasks: no tasks marked as completed");
                }
            }
            _ => unreachable!(),
        },
    }
    match task_stack.write_to_file(TASKS_FILE) {
        Ok(_) => (),
        Err(e) => println!("unable to save changes: {}", e),
    };
}
