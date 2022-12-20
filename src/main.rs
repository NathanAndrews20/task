use clap::{Parser, Subcommand};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind},
};

const TASKS_FILE: &str = "tasks.txt";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]

struct Cli {
    #[command(subcommand)]
    command: Commands,
    // pattern: String,
    // path: std::path::PathBuf,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// add a task
    Add { content: String },

    /// list all tasks
    List,

    /// mark a task as complete
    Complete { number: i32 },

    /// remove a task
    Remove { number: i32 },
}

#[derive(Debug)]
struct Tasks {
    map: HashMap<usize, Task>,
}

#[derive(Debug)]
struct Task {
    content: String,
    completed: bool,
}

fn main() {
    let tasks = match File::open(TASKS_FILE) {
        Ok(file) => match parse_tasks(file) {
            Ok(tasks) => tasks,
            Err(e) => {
                println!("error parsing tasks: {}", e);
                return;
            }
        },
        Err(_) => Tasks {
            map: HashMap::new(),
        },
    };

    let args = Cli::parse();
    match args.command {
        Commands::Add { content } => handle_add_task(content),
        Commands::List => handle_list_tasks(),
        Commands::Complete { number } => handle_complete_task(number),
        Commands::Remove { number } => handle_remove_task(number),
    }
}

fn parse_tasks(file: File) -> Result<Tasks, Error> {
    let mut map: HashMap<usize, Task> = HashMap::new();

    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                let (num, completed, content) = match parse_task(line.clone()) {
                    Ok(tuple) => tuple,
                    Err(e) => return Err(e),
                };
                map.insert(num, Task { content, completed });
                line.clear()
            }
            Err(e) => return Err(e),
        }
    }

    return Ok(Tasks { map });
}

fn parse_task(line: String) -> Result<(usize, bool, String), Error> {
    let task_data_vec: Vec<&str> = line.splitn(3, ",").collect();

    let task_num = match task_data_vec[0].parse::<usize>() {
        Ok(b) => b,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };

    let task_completed = match task_data_vec[1].parse::<bool>() {
        Ok(b) => b,
        Err(e) => return Err(Error::new(ErrorKind::InvalidData, e)),
    };

    let task_content = task_data_vec[2].to_string();

    return Ok((task_num, task_completed, task_content));
}

fn handle_add_task(_content: String) {}
fn handle_list_tasks() {}
fn handle_complete_task(_number: i32) {}
fn handle_remove_task(_number: i32) {}
