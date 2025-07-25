use crate::todo::core::TodoItem;

pub fn create_todo(todos: &mut Vec<TodoItem>, title: Option<String>, content: Option<String>) {
    let mut inputs: Vec<String> = Vec::new();

    if let Some(arg_title) = title {
        if !arg_title.is_empty() {
            inputs.push(arg_title);
        }
    }

    if let Some(arg_content) = content {
        if !arg_content.is_empty() {
            inputs.push(arg_content);
        }
    }

    // match title {
    //     Some(arg_title) => {
    //         if !arg_title.is_empty() {
    //             inputs.push(arg_title);
    //         }
    //     }
    //     _ => {}
    // }
    //
    // match content {
    //     Some(arg_content) => {
    //         if !arg_content.is_empty() {
    //             inputs.push(arg_content);
    //         }
    //     }
    //     _ => {}
    // }

    let mut ok = inputs.len() == 0;
    while ok {
        let len = inputs.len();

        if len == 0 {
            println!("Please input todo title");

            let mut title = String::new();

            std::io::stdin()
                .read_line(&mut title)
                .expect("read line failed");

            if title.trim().is_empty() {
                continue;
            }

            inputs.push(title.trim().to_string());
        } else if len == 1 {
            println!("Please input todo content");

            let mut content = String::new();

            std::io::stdin()
                .read_line(&mut content)
                .expect("read line failed");

            if content.trim().is_empty() {
                continue;
            }

            inputs.push(content.trim().to_string());
        } else {
            println!("title: [{}]", inputs[0].clone());
            println!("content: [{}]", inputs[1].clone());
            println!("Are you sure to crate this todo? (y/n)");

            let mut sure = String::new();

            std::io::stdin()
                .read_line(&mut sure)
                .expect("read line failed");

            if sure.trim().to_lowercase() != "n" {
                ok = false;
            } else {
                inputs.clear();
            }
        }
    }

    let inputs_len = inputs.len();

    let title = if inputs_len > 0 {
        inputs[0].clone()
    } else {
        String::from("default title")
    };

    let content = if inputs_len > 1 {
        inputs[1].clone()
    } else {
        String::from("default content")
    };

    todos.push(TodoItem::new(title.as_str(), content.as_str()));

    println!("create todo title: {}, content: {}", title, content);
}
