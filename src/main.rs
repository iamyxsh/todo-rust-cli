use clap::{Parser, Subcommand};
use colored::Colorize;
use postgres::{Client, NoTls};

#[derive(Parser)]
#[clap()]
#[clap(name = "TodoList", about = "A persistent todo-list", long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(arg_required_else_help = true)]
    Add { item: String },

    #[clap(arg_required_else_help = true)]
    Remove { id: i32 },

    #[clap(arg_required_else_help = true)]
    MarkCompleted { id: i32 },

    #[clap(arg_required_else_help = true)]
    MarkUncompleted { id: i32 },

    #[clap()]
    GetAll,
}

fn main() {
    let mut client =
        Client::connect("postgres://postgres:docker@127.0.0.1:5432/postgres", NoTls).unwrap();

    client
        .batch_execute(
            "
    CREATE TABLE IF NOT EXISTS todo (
        id          SERIAL PRIMARY KEY,
        task        TEXT NOT NULL,
        completed   BOOLEAN
    )
",
        )
        .unwrap();

    let args = Cli::parse();

    match &args.command {
        Commands::GetAll => {
            let todos = client
                .query("SELECT id, task, completed from todo", &[])
                .unwrap();

            println!("");
            println!(
                "{}    {}     {}",
                "ID".yellow(),
                "Completed".green(),
                "Task".red()
            );
            println!("");
            println!("");
            for row in todos.into_iter().rev() {
                let ind: i32 = row.get(0);
                let task: String = row.get(1);
                let completed: String = if row.get(2) {
                    "x".to_string()
                } else {
                    " ".to_string()
                };
                println!(
                    "{}        [{}]        {}",
                    (ind).to_string().yellow(),
                    completed.green(),
                    task.red()
                );
                println!("");
            }
        }
        Commands::Add { item } => {
            client
                .query(
                    "INSERT INTO todo (task, completed) VALUES ($1, $2)",
                    &[&item, &false],
                )
                .unwrap();
            println!("");
            println!("{}", "Successfully added.".green());
            println!("");
        }
        Commands::Remove { id } => {
            client
                .query("DELETE FROM todo WHERE id = $1", &[&id])
                .unwrap();

            println!("");
            println!("{}", "Successfully deleted.".green());
            println!("");
        }
        Commands::MarkCompleted { id } => {
            client
                .query("UPDATE todo SET completed = true WHERE id = $1", &[&id])
                .unwrap();

            println!("");
            println!("{}", "Successfully mark completed.".green());
            println!("");
        }
        Commands::MarkUncompleted { id } => {
            client
                .query("UPDATE todo SET completed = false WHERE id = $1", &[&id])
                .unwrap();

            println!("");
            println!("{}", "Successfully mark uncompleted.".green());
            println!("");
        }
    }
}
