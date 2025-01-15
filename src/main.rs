use clap::{Parser, Subcommand};

struct Todo {
    id: u32,
    content: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { content: String },
    Remove { id: u32 },
}

fn main() {
    let mut todos = vec![Todo { id : 1, content: String::from("To do Todo List")}];

    let args = Cli::parse();

    match args.command {
        Commands::Add { content } => todos.push(Todo {content, id: todos.len() as u32 + 1}),
        Commands::Remove { id } => todos = todos.into_iter().filter(|it| it.id == id).collect(),
    }

    for todo in todos {
        println!("{}: {}", todo.id, todo.content);
    }
}
