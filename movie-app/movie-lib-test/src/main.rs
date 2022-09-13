/*fn main() {
    println!("Hello, world!");
}*/

extern crate movie_lib;
use movie_lib::movies::play;
fn main() {
   println!("inside main of test ");
   play("Tutorialspoint".to_string())
}
