fn main() {
    hello("Hello World");
    let num1: i32 = 50;
    let num2: i32 = 78;
    let sum_val = sum(num1, num2);

    println!("The sum of {} and {} is {}", num1, num2, sum_val);

    person("Vaishnav", 20, 180.6);
}

fn hello(text: &str){
    println!("{}", text);
}

fn sum(x: i32, y: i32) -> i32 {
    x + y     //OR return x + y;
}

fn person(name: &str, age: i32, height: f64){
    println!("I am {} and I am {} years old. My height is {}", name, age, height);
}
