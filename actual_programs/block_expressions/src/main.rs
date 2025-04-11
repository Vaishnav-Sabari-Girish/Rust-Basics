fn main() {
    println!("All of same datatype");
    let x = {
        let num1: i32 = 67;
        let num2: i32 = 78;
        num1 + num2 
    };

    println!("Value of x: {}", x);

    println!("");

    println!("Different datatypes");
    let y = {
        let cost: f64 = 100.5;
        let qty: i32 = 10;
        let tax_rate: f64 = 0.2;
        let subtotal = cost * qty as f64;   //Converting to f64 
        subtotal + (subtotal + tax_rate)
    };

    println!("The value of y is {}", y);
}
