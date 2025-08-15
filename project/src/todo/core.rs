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
    List {
        #[arg(short, long)]
        title: Option<String>,
        #[arg(short, long)]
        content: Option<String>,
    },
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

pub struct TodoItemFilter {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl TodoItemFilter {
    pub fn new() -> Self {
        Self {
            title: Option::None,
            content: Option::None,
        }
    }

    // 1st generic option
    pub fn set_title<T: Into<String>>(&mut self, title: T) {
        self.title = Some(title.into());
    }

    pub fn set_content<T: Into<String>>(&mut self, content: T) {
        self.content = Some(content.into());
    }

    // 2nd generic option
    // pub fn set_title<T>(&mut self, title: T) where T: Into<String> {
    //     self.title = Some(title.into());
    // }
    //
    // pub fn set_content<T>(&mut self, content: T) where T: Into<String> {
    //     self.content = Some(content.into());
    // }

    pub fn filter(&self, list: &Vec<TodoItem>) {
        let mut filterd_list = Vec::<&TodoItem>::new();

        if self.title.is_none() && self.content.is_none() {
            for item in list {
                filterd_list.push(item);
            }
        } else {
            for item in list {
                let mut flag: (bool, bool) = (false, false);

                flag.0 = match &self.title {
                    Some(title) => item.title.contains(title),
                    _ => true,
                };

                flag.1 = match &self.content {
                    Some(content) => item.content.contains(content),
                    _ => true,
                };

                if flag.0 && flag.1 {
                    filterd_list.push(item);
                }
            }
        }

        for item in filterd_list {
            println!("todo title: {}, content: {}", item.title, item.content);
        }
    }
}
