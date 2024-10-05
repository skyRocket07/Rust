// CARGO
// => Cargo is Rust’s build system and package manager.
//    Most Rustaceans use this tool to manage their Rust projects because 
//    Cargo handles a lot of tasks for you, such as building your code, 
//    downloading the libraries your code depends on, and building those libraries. 
//    (We call the libraries that your code needs dependencies.)


// IMPORTANT POINTS
// CARGO NEW => We can create a project using cargo new.
// CARGO BUILD => We can build a project using cargo build.
// CARGO RUN => We can build and run a project in one step using cargo run.
// CARGO CHECK => We can build a project without producing a binary to check for errors using cargo check.
// Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.
// RELEASE => cargo build --release => stores in target/release

fn main() {
    println!("Hello, world!");
}
