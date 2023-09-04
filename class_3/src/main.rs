fn main() {
    // *const* is a constant and CANNOT be redefined or reassigned
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("SECONDS_IN_MINUTE: {}", SECONDS_IN_MINUTE);

    // **let** defines an inferred immutable variable 
    let mut x: u32 = 4;
    println!("x is : {}", x);

    {
        let x = x - 1;
        println!("x is : {}", x);
    }

    {
        let x = 2;
        println!("x is : {}", x);
    }

    let mut x: u32 = x + 1;
    println!("x is : {}", x);

    {
        x = 2;
        println!("x is : {}", x);
    }

    let mut x: u32 = x + 1;
    println!("x is : {}", x);
}
