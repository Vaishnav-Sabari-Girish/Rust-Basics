use std::io;

fn main() {
    let mut input: String = String::new();
    println!("Enter the number: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error Reading Input");

    let n: i32 = input.trim().parse().expect("Enter valid number");

    let result = fibonacci(n);

    println!("Fibonacci of {} numbers is {}", n, result);
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 && n >= 0 {
        n 
    }
    else{
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
