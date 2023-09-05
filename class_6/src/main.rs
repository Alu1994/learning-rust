use std::io;

fn main() {
    let x = 127_000 as i64;  
    let y = 10_i32;   

    let z = x / (y as i64);
    println!("{}", z);

    {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("expected to read line");

        let int_input: i64 = input.trim().parse().unwrap();

        println!("{}", input);
    }
}
