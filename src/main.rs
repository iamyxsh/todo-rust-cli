mod database;
mod utility;

use clap::{Parser, Subcommand};

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
    let mut client = database::return_client();

    database::create_table(&mut client);

    let args = Cli::parse();

    match &args.command {
        Commands::GetAll => {
            let todos = database::get_all_todos(&mut client);

            utility::print_todo(todos)
        }
        Commands::Add { item } => {
            let result = utility::return_result(database::add_todo(&mut client, item.to_string()));

            utility::print_result(result);
        }
        Commands::Remove { id } => {
            let result = utility::return_result(database::remove_todo(&mut client, *id));

            utility::print_result(result);
        }
        Commands::MarkCompleted { id } => {
            let result = utility::return_result(database::mark_completed(&mut client, *id));

            utility::print_result(result);
        }
        Commands::MarkUncompleted { id } => {
            let result = utility::return_result(database::mark_uncompleted(&mut client, *id));

            utility::print_result(result);
        }
    }
}
