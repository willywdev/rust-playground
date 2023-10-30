use std::io;
// use = bring a library into scope
// std = standard library
// io = input/output library
// :: = namespace operator (it's like a folder structure)

fn main() {
    println!("Please pick one of the functions to get more information!");
    guessing_game();
}
// Rust is an ahead-of-time compiled language,
// meaning you can compile a program and give the executable to someone else,
// and they can run it even without having Rust installed.
// -----------------------------------------------------------------------------
// -----------------------------------------------------------------------------

// Hello Cargo!
// -----------------------------------------------------------------------------
// Cargo is Rust’s build system and package manager.
// Cargo is installed with Rust.
// Cargo has commands for building, testing, and managing Rust projects, and
// Cargo downloads and builds your project’s dependencies.
// println! -> macro
// (!) -> indicates that it's a macro and not a function
// (macro is a metaprogramming technique that allows the programmer to generate code at compile time)

/* fn hello_cargo() {
    println!("To initiate a new Cargo project you can use:");
    println!("cargo new <project_name>");
    println!("This will create a Cargo.toml, a git repository, and a src directory with a main.rs file inside.");
    println!("Rust-Analyzer depends on a Cargo.toml and main.rs in /src/ to work properly.");
} */

// -----------------------------------------------------------------------------
/* fn cargo_build() {
    println!("To build a Cargo project you can use:");
    println!("cargo build");
    println!("If anything goes wrong, the Compiler is gonna tell you.");
    println!("For example: Functions that are not used.");
    println!("Cargo then creates an executable in /target/debug/.");
    println!("You can run the executable with: ./<project_name>");
} */

/* fn cargo_information() {
    println!("Cargo assumes all your code lives in the src directory in the root of your project.");
    println!("Cargo assumes that your root directory is for configuration files, READMEs, and anything else not related to your code.");
    println!(
        "With cargo check you can check if your code compiles without creating an executable."
    );
} */

fn guessing_game() {
    println!("Guess the number:");
    let mut _guess = String::new();
    // let = create a variable
    // mut = mutable (variable can be changed), default: immutable (variables cannot be changed)
    // String::new() = create a new instance of a String
    // :: = associated function (static method) Like object.func() in JS
    // We need to a new instance because Strings are a complex data structure,
    // and so we need to allocate memory on the heap. Without allocating memory,
    // the compiler wouldn’t know how much space to reserve for the string and we
    // couldn’t store our data in it.
    io::stdin()
        .read_line(&mut _guess)
        .expect("Failed to read line.");

    println!("You guessed {_guess}");
}
