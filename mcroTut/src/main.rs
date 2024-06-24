macro_rules! print_chars_with_index {
    ($s:expr) => {
        for (index, c) in $s.chars().enumerate() {
            println!("{}: {}", index, c);
        }
    };
}
fn main() {
    let vec_one:Vec<u32> = vec![1,3,5,6,7,8,9,10,11,12,13];
    println!("Hello, world! {}",vec_one[6]);

    let my_string = "Hello, Rust!";
    // Use the macro
    print_chars_with_index!(my_string);

}
