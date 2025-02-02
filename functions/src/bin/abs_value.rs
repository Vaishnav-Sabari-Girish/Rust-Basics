fn abs_val(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        -x
    }
}
fn main() {
    let num: f64 = -265.23;
    let abs_num: f64 = abs_val(num);
    println!("Absolute Value of {} is {}", num, abs_num);
}
