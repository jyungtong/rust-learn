use crate::todo::core::TodoItemFilter;

use super::core::TodoItem;

pub fn list_todo(todos: &Vec<TodoItem>, title: Option<String>, content: Option<String>) {
    // for todo in todos {
    //     println!("todo title: {}, content: {}", todo.title, todo.content);
    // }

    let mut filter = TodoItemFilter::new();

    if let Some(title) = title {
        filter.set_title(title);
    }

    if let Some(content) = content {
        filter.set_content(content);
    }

    filter.filter(todos);
}
