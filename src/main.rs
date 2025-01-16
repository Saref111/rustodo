use std::cmp::Ordering;

use clap::{Parser, Subcommand};
use db::{add_todo, done_todo, get_todos, init_db, remove_todo};
use rusqlite::Result as DBResult;

mod db;

struct Todo {
    id: u32,
    content: String,
    done: bool,
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
    Done { id: u32 },
    Sort
}

fn main() -> DBResult<()> {
    let conn = init_db()?;

    let args = Cli::parse();
    let mut sort_by_done = false;

    if let Some(command) = args.command {
        match command {
            Commands::Add { content } => add_todo(&conn, content)?,
            Commands::Remove { id } => remove_todo(&conn, id)?,
            Commands::Done { id } => done_todo(&conn, id)?,
            Commands::Sort => {
                sort_by_done = true
            }
        }
    }

    let mut todos = get_todos(&conn)?;

    if sort_by_done {
        todos.sort_by(|a, b| {
            if a.done && !b.done {
                Ordering::Greater
            } else if !a.done && b.done {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        });
    }

    for (i, todo) in todos.into_iter().enumerate() {
        let string_to_print = if todo.done { 
            console::style(todo.content).strikethrough().to_string()
        } else { 
            todo.content 
        }; 
        println!("{}: {} (id: {})", i + 1, string_to_print, todo.id);
    }

    Ok(())
}
