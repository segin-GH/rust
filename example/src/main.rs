fn main() {
    println!("What is your name?");

    let mut name: String = String::new();
    let greeting: &str = "Hello, ";

    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("{}{}", greeting, name.trim_end());
}
