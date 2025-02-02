fn factorial(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}
fn main() {
    let num: i64 = 10;
    let fact_num: i64 = factorial(num);
    println!("The factorial of {} is {}", num, fact_num);
}
