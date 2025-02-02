fn main() {
    for i in 0..5 {
        let even_odd = if i % 2 == 0 {
            format!("{} is even", i)
        } else {
            format!("{} is odd", i)
        };
        println!("{}", even_odd)
    }
}
