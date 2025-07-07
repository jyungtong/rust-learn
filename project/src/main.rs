fn main() {
    let args: Vec<String> = std::env::args().collect();
    let lens = args.len();
    let title = args[1].clone();
    // let mut content = String::from("default content");
    //
    // if lens > 2 {
    //     content = args[2].clone();
    // }

    let content = if lens > 2 {
        println!("has more than 2");
        args[2].clone()
    } else {
        String::from("default content")
    };

    println!("todo title: {}, content: {}", title, content);
}
