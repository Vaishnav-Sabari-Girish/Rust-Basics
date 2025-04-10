fn main() {
    println!("Primitive Data Types");
    println!("");

    println!("Integers");
    let x: i32 = -42;     //Signed 32 bit integer 
    let y: u64 = 100;     //Unsigned 64 bit integer 
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
    println!("");

    println!("Float");
    let pi: f64 = 3.14;
    println!("Value of PI: {}", pi);
    println!("");

    println!("Boolean values");
    let is_student: bool = true;
    let is_working: bool = false;

    println!("Is he a student: {}", is_student);
    println!("Is he working: {}", is_working);
    println!("");

    println!("Characters");
    let letter: char = 'a';
    println!("The letter given is : {}", letter);
    println!("");
}
