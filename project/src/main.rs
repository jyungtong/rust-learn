use crate::todo::storage::{read_todo_list, save_todo_list};
use clap::Parser;
use todo::core::TodoCommand;

mod todo;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Todo cli")]
struct Program {
    #[command(subcommand)]
    pub command: TodoCommand,
}

fn main() {
    let args = Program::parse();
    let save_file = "todo.json";
    let mut todos = read_todo_list(save_file);

    match args.command {
        TodoCommand::Create => todo::create::create_todo(&mut todos),
        TodoCommand::List => todo::list::list_todo(&todos),
    }

    save_todo_list(save_file, &todos);
}
