fn main() {
    let mut sum = 0;

    //Adding all numbers between the range start and stop
    let start = 0;
    let stop = 10;

    for i in start..stop {
        sum += i;
    }

    println!(
        "The sum of all numbers in between {} and {} is {}",
        start, stop, sum
    );
}
