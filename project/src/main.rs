// fn main() {
//     let s = "hello world";
//
//     println!("{}", &s[..4]);
// }

struct TodoItem {
    title: String,
    content: String,
}

fn create_todo_item(title: &str, content: &str) -> TodoItem {
    TodoItem {
        title: title.to_string(),
        content: content.to_string(),
    }
}

fn main() {
    let mut todos: Vec<TodoItem> = Vec::new();
    todos.push(create_todo_item("learn rust", "read rust book"));
    todos.push(create_todo_item("work", "do something"));
    todos.push(create_todo_item("play", "what game"));

    let mut inputs: Vec<String> = Vec::new();
    let args: Vec<String> = std::env::args().collect();

    match args[1].clone().as_str() {
        "create" => {
            loop {
                let len = inputs.len();
                let mut ok = true;

                if len == 0 {
                    println!("Please input todo title");

                    let mut title = String::new();

                    std::io::stdin()
                        .read_line(&mut title)
                        .expect("read line failed");

                    if title.is_empty() {
                        println!("emptyyyy");
                        continue;
                    }

                    inputs.push(title.trim().to_string());
                } else if len == 1 {
                    println!("Please input todo content");

                    let mut content = String::new();

                    std::io::stdin()
                        .read_line(&mut content)
                        .expect("read line failed");

                    if content.is_empty() {
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

                    if !ok {
                        break;
                    }
                }
            }

            let title = inputs[0].clone();
            let content = inputs[1].clone();

            println!("create todo title: {}, content: {}", title, content);
        }

        "l" | "list" => {
            for todo in todos {
                println!("todo title: {}, content: {}", todo.title, todo.content);
            }
        }

        _ => {
            println!("unknown command")
        }
    }
}
