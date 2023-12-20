mod repl;
mod tests;
mod token;

fn main() {
    println!("Hello, gorilla!");
    repl::repl::start();
}
