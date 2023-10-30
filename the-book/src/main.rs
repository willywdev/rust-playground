// fn -> function
// main -> entry point

fn main() {
    cargo_build()
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
fn cargo_build() {
    println!("To build a Cargo project you can use:");
    println!("cargo build");
    println!("If anything goes wrong, the Compiler is gonna tell you.");
    println!("For example: Functions that are not used.");
    println!("Cargo then creates an executable in /target/debug/.");
    println!("You can run the executable with: ./<project_name>");
}
