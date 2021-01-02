mod utils;

fn main() {
    loop {
        let expr = utils::io::read_line("> ");
        println!("You entered: {}", expr);
    }
}
