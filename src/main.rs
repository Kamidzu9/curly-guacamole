use std::io;

fn main() {
    println!("Hello, world!");

    // Prompt for first number
    println!("Enter the first number:");
    let mut input1 = String::new();
    io::stdin()
        .read_line(&mut input1)
        .expect("Failed to read line");
    let a: i32 = input1.trim().parse().expect("Please enter a valid number");

    // Prompt for second number
    println!("Enter the second number:");
    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");
    let b: i32 = input2.trim().parse().expect("Please enter a valid number");

    let result = not_add(a, b);
    println!("Result: {}", result);
}

fn not_add(a: i32, b: i32) -> i32 {
    a + b
}
