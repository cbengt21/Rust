# Rust
Learning Rust, https://www.rust-lang.org/

This is were my main reading starts:
https://www.tutorialspoint.com/rust/index.htm
https://doc.rust-lang.org/book/title-page.html
https://doc.rust-lang.org/reference/introduction.html

- compile apps by "rustc file_name.rs" or optionally add your own name by "rustc file_name.rs -o exe_name"
- run app by "./exe_name"

Generating a new project (https://www.rust-lang.org/learn/get-started):
"cargo new 'project_name'"

Building a project (without running it):
"cargo build"

Running project (also builds it if not done before):
"cargo run"

Updating crates (will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file.):
"cargo update"

Build documentation provided by all of your dependencies locally and open it in your browser:
"cargo doc --open"
