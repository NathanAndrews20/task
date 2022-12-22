use std::{ffi::OsString, fs, io::ErrorKind, path::Path};

use ansi_term::Style;
use clap::{ArgGroup, Parser, Subcommand};

use indicatif::{ProgressBar, ProgressStyle};
use task_stack::TaskStack;

mod task_stack;

const TASKS_DIR: &str = ".tasks";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// create a task group
    CreateGroup { group_name: OsString },

    /// add a task
    Add {
        group_name: OsString,
        content: Vec<String>,
    },

    /// list all tasks
    #[command(group(ArgGroup::new("list").required(true).args(["group_name", "all"])))]
    List {
        #[arg(short = 'a', long = "all")]
        all: bool,

        group_name: Option<OsString>,

        #[arg(short = 'p', long = "progress")]
        progress: bool,
    },

    /// mark a task as complete
    Complete { group_name: OsString, number: isize },

    /// remove a task
    #[command(group(ArgGroup::new("remove").required(true).args(["number", "completed"])))]
    Remove {
        group_name: OsString,

        #[arg(short = 'n', long = "number")]
        number: Option<isize>,

        #[arg(short = 'c', long = "completed")]
        completed: bool,
    },
}

fn main() {
    // create the .tasks directory if it does not exists
    match fs::create_dir_all(Path::new(TASKS_DIR)) {
        Ok(_) => (),
        Err(e) => {
            println!("unable to load tasks: {e}");
            return;
        }
    };

    // parse the CLI
    let args = Cli::parse();

    // handle the commands
    let result: String = match args.command {
        Commands::CreateGroup { group_name } => with_task_stack(true, group_name.clone(), |_| {
            let name_to_print = match group_name.to_str() {
                Some(str) => str,
                None => "[task-group]",
            };
            format!("created task group: {}", name_to_print)
        }),

        Commands::Add {
            content: raw_content,
            group_name,
        } => with_task_stack(true, group_name, |task_stack| {
            let content = raw_content.join(" ").trim().to_string();
            if !content.is_empty() {
                task_stack.add(content);
                return String::new();
            } else {
                return String::from("cannot add empty task");
            }
        }),

        Commands::List {
            all,
            group_name,
            progress,
        } => match (group_name, all, progress) {
            (None, true, _) => {
                let files = match fs::read_dir(TASKS_DIR) {
                    Ok(f) => f,
                    Err(e) => {
                        println!("unable to list task progress: {e}");
                        return;
                    }
                };
                for entry in files {
                    match entry {
                        Ok(file) => with_task_stack(false, file.file_name(), |task_stack| {
                            print_progress(task_stack);
                            return String::new();
                        }),
                        Err(_) => todo!(),
                    };
                }
                String::new()
            }
            (Some(group_name), false, true) => with_task_stack(false, group_name, |task_stack| {
                print_progress(task_stack);
                String::new()
            }),
            (Some(group_name), false, false) => with_task_stack(false, group_name, |task_stack| {
                print_list(task_stack);
                String::new()
            }),
            _ => unreachable!(),
        },

        Commands::Complete {
            number: task_number,
            group_name,
        } => with_task_stack(false, group_name, |task_stack| {
            if task_number < 1 {
                return format!(
                    "unable to mark task as completed: no task with number {task_number}"
                );
            }
            match task_stack.complete((task_number - 1) as usize) {
                Ok(_) => String::new(),
                Err(e) => return format!("unable to mark task as completed: {}", e),
            }
        }),

        Commands::Remove {
            number: number_option,
            completed,
            group_name,
        } => with_task_stack(false, group_name, |task_stack| {
            match (number_option, completed) {
                (Some(task_number), _) => {
                    if task_number < 1 {
                        return format!("unable to remove task: no task with number {task_number}");
                    }
                    match task_stack.remove((task_number - 1) as usize) {
                        Ok(_) => String::new(),
                        Err(e) => format!("unable to remove task: {}", e),
                    }
                }
                (_, true) => {
                    if !task_stack.remove_completed() {
                        return format!("unable to remove tasks: no tasks marked as completed");
                    } else {
                        String::new()
                    }
                }
                _ => unreachable!(),
            }
        }),
    };
    if !result.is_empty() {
        println!("{result}");
    }
}

fn with_task_stack(
    create_if_not_exists: bool,
    group_name: OsString,
    closure: impl Fn(&mut TaskStack) -> String,
) -> String {
    let path_buf = Path::new(TASKS_DIR).join(Path::new(group_name.as_os_str()));
    let mut task_stack = match TaskStack::from_file(path_buf.as_path()) {
        Ok(ts) => ts,
        Err(e) => {
            if e.kind().eq(&ErrorKind::NotFound) {}
            match (e.kind(), create_if_not_exists) {
                (ErrorKind::NotFound, true) => TaskStack::new(),
                (ErrorKind::NotFound, false) => return format!("{e}"),
                _ => return format!("unable to load tasks: {e}"),
            }
        }
    };
    let result = closure(&mut task_stack);

    match task_stack.write_to_file(path_buf.as_path()) {
        Ok(_) => (),
        Err(e) => return format!("unable to save changes: {}", e),
    };

    return result;
}

fn print_list(task_stack: &TaskStack) {
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

fn print_progress(task_stack: &TaskStack) {
    println!(
        "total tasks: {}, completed tasks: {}, tasks remaining: {}",
        task_stack.num_tasks(),
        task_stack.num_tasks_completed(),
        task_stack.num_tasks() - task_stack.num_tasks_completed()
    );
    let bar = ProgressBar::new(task_stack.num_tasks() as u64);
    bar.set_message(task_stack.name());
    bar.set_style(
        ProgressStyle::with_template("{msg}: [{bar:60.cyan/black}] {pos}/{len}")
            .unwrap()
            .progress_chars("#|-"),
    );
    bar.inc(task_stack.num_tasks_completed() as u64);
    bar.abandon();
}
