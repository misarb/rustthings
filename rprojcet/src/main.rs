use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("faild to read the Input");
    let int_input: i64 = input.trim().parse().unwrap();

    if int_input % 2 == 0 {
        println!("Pair");
    } else {
        println!("impair")
    }

    print!("Hello world");
}
