# Basics of Rust Part - 2 

## Block Expressions 

These allows you to calculate the value of a variable using expressions . 

```rs 
fn main(){
  let x = {
    let num1: i32 = 10;
    let num2: i32 = 45;
    num1 + num2           //OR return num1 + num2;
  };
}
```

It can also have different datatypes

```rs 
fn main(){
  let y = {
    let cost: f64 = 100.5;
    let qty: i32 = 10;
    let tax_rate: f64 = 0.2; 
    let subtotal = cost * qty as f64;      //Converting to f64 
    subtotal + (subtotal + tax_rate)     //or return subtotal + (subtotal + tax_rate);
  }
}
```

---

## Functions 

They are reusable block of code that perform a certain task 

```rs 
fn main(){
  hello("Hello World");
  hello("Nice to meet you");
}

fn hello(word: &str){
  println!("{}", word);
}
```

Functions can also return values 

```rs 
fn main(){
  let a: i32 = 10;
  let b: i32 = 50;

  let y: i32 = sum(a, b);

  println!("{}", y);
}

fn sum(num1: i32, num2: i32) -> i32 {
  num1 + num2         //OR return num1 + num2;
}
```

---

# Fibonacci Series 

---

This question also include user input 

```rs 
use std::io;

fn main(){
  let mut input: String = String::new();   //empty string
  
  io::stdin()
      .read_line(&mut input)
      .expect("Error reading input");

  let n: i32 = input.trim().parse().expect("Enter valid number");  //Convering to i32 

  let result = fibonacci(n);

  println!("Result of fibonacci Series: {}", result);
}

fn fibonacci(n: i32) -> i32 {
  if n <= 1{
      n            //OR return n;
  }
  else{
    fibonacci(n - 1) + fibonacci(n - 2)    //OR return fibonacci(n - 1) + fibonacci(n - 2);
  }

}
```

---

## Statements vs Expressions 

---


|Aspect|Statements|Expressions|
|---|---|---|
|**Definition**|Instructions that perform actions but do not return a value| Code that evaluates an action and returns a value|
|**Examples**|`let x = 5;`|`5 + 3`, `x * 2`|
|**Semicolon**|Used|Can be used even without semicolon|
|**Code Examples**|`let x = 5;`|`y = add(x, y)`|

The code for the above can be as follows 

```rs 
fn main(){
    let x = 13;    //Statement
    let y = {
      let a = 67;
      let b = 78;
      a + b       // Expression 
  }

  let z = add(x, y);
}

fn add(num1: i32, num2: i32){
  num1 + num2    //Expression 
}
```
