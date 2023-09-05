fn main() {
    test_one();
    add_numbers(20, 30);

    let result = add_numbers_2(2, 3);
    println!("result: {}", result);

    let result_add_numbers_3 = add_numbers_3(12, 3);
    println!("result_add_numbers_3: {}", result_add_numbers_3);

    let number = {
        let x = 3;
        x + 1
    };
    println!("number: {}", number);
}

fn test_one() {
    println!("Test has been called...");
}

fn add_numbers(x: i32, y: i32) {
    println!("The sum is: {}", x + y);
}

fn add_numbers_2(x: i32, y: i32) -> i32 {
    return x + y;
}

fn add_numbers_3(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 10 {
        return result - 10;
    }
    result
}