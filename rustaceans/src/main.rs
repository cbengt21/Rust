fn main() {
    const EMOJI:char = '😁';
    let mut empty_string = String::new();

    println!("Hello, world! {}", EMOJI);

    println!("length of empty_string is {}",empty_string.len());
    println!("empty_string is {}",empty_string);
    empty_string.push_str("hello");
    println!("length of empty_string is {}",empty_string.len());
    println!("empty_string is {}",empty_string);

    fn_hello();
}


fn fn_hello(){
    println!("hello from function fn_hello ");
 }
