use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Subcommand)]
pub enum TodoCommand {
    Create {
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
    List,
}

impl TodoItem {
    pub fn new(title: &str, content: &str) -> Self {
        create_todo_item(title, content)
    }

    pub fn serializer(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn deserializer(s: &str) {
        serde_json::from_str(s).unwrap()
    }
}

pub fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
        title: title.to_string(),
        content: content.to_string(),
    }
}

pub fn get_todo_list() -> Vec<TodoItem> {
    let mut todos: Vec<TodoItem> = Vec::new();

    todos.push(create_todo_item("learn rust", "read rust book"));
    todos.push(create_todo_item("work", "do something"));
    todos.push(create_todo_item("play", "what game"));

    return todos;
}
