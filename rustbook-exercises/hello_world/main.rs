fn main() {
    greetings();
    bye();
}


fn greetings(){
    println!("Namaste Rust");
}

fn bye(){
    println!("Tata Bye Bye");
}

// NOTES
// 1. STARTS WITH MAIN 
// 2. println!() is macro | macro contains => ! and are different from functions
// 3. Compile + Execute the executable => ahead of time compilation
// 4. rustc main.rs => compiles main.rs and generates binary executable main which can be run on any system even with rust installed on installed