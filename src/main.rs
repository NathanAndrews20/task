use clap::{Parser, Subcommand};

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

fn main() {
    let args = Cli::parse();
    match args.command {
        Commands::Add { content } => handle_add_task(content),
        Commands::List => handle_list_tasks(),
        Commands::Complete { number } => handle_complete_task(number),
        Commands::Remove { number } => handle_remove_task(number),
    }
}

fn handle_add_task(content: String) {
    
}

fn handle_list_tasks() {
    
}

fn handle_complete_task(number: i32) {
    
}
fn handle_remove_task(number: i32) {
    
}
