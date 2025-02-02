fn sqr(x: f64) -> f64 {
    x * x
}
fn main() {
    let num: f64 = 42.0;
    let sqr_num = sqr(num);
    println!("The square of {} is {}", num, sqr_num);
}
