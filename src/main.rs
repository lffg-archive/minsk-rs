use std::io::{self, Write};

fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input
}

fn main() {
    loop {
        let expr = read_line("> ");
        println!("You entered: {}", expr);
    }
}
