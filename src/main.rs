use clap::{Parser, Subcommand};
use db::{add_todo, get_todos, init_db, remove_todo};
use rusqlite::Result as DBResult;

mod db;

struct Todo {
    id: u32,
    content: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Add { content: String },
    Remove { id: u32 },
}

fn main() -> DBResult<()> {
    let conn = init_db()?;

    let args = Cli::parse();

    if let Some(command) = args.command {
        match command {
            Commands::Add { content } => add_todo(&conn, content)?,
            Commands::Remove { id } => remove_todo(&conn, id)?,
        }
    } 

    let todos = get_todos(&conn)?;

    for todo in todos {
        println!("{}: {}", todo.id, todo.content);
    }

    Ok(())
}
