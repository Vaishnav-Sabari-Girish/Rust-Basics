# Basics of Rust 

## Hello World 

```rs 
fn main() {
  println!("Hello World");
}
```

---

## Variables 

```rs 
fn main() {
  let var1: i32 = 10;   // 32 bit integer (immutable variable)
  let mut var2: i32 = 40;  //32 bit integer (mutable variable)
}
``` 

---

## User input

```rs 
use std::io;

fn main() {
  let mut name: String = String::new();  //Empty String 
  io::stdin().read_line(&mut name).expect("Error in reading input");

  println!("Hello {}", name);
}
```

---

## Range of values 

```rs 
fn main(){
  let x: i32 = 10;

  for i in 0..10{
      println!("{} + {} = {}", x, i, x+i);
  }
}
```
