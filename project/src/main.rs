fn main() {
    let mut inputs: Vec<String> = Vec::new();
    let args: Vec<String> = std::env::args().collect();
    let mut ok = args[1].clone() == "create";

    while ok {
        let len = inputs.len();

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
        }
    }

    let title = inputs[0].clone();
    let content = inputs[1].clone();

    println!("create todo title: {}, content: {}", title, content);
}
