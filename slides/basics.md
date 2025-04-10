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

---

## Datatypes 

### Primitive Datatypes 

1. Integer
    1. Signed (`i8, i16, i32, i64, i128`)
    2. Unsigned (`u8, u16, u32, u64, u128`)
2. Float (`f32, f64`)
3. Boolean (`bool`)
4. Character (`char`)

```rs 
fn main() {
  //Integers
  let x: i32 = -42;  //Signed
  let y: u64 = 100;  //Unsigned

  //Float
  let pi: f64 = 3.14;
  
  //Boolean 
  let is_studennt: bool = true;
  let is_working: bool = false;

  //Character
  let letter: char = 'a';
}
```

---

### Compound Datatypes 

1. Array 
2. Tuples 
3. Slices 

```rs 
fn main(){
  //Arrays (Same Datatype elements)
  let numbers: [i32, 5] = [1, 2, 3, 4, 5];  //let <variable_name>: <datatype, size> = <value>
  let fruits: [&str, 3] = ["Apple", "Banana", "Orange"];

  //Tuples (Multiple datatypes including Arrays)
  let tuple: (String, i32, bool, [i32, 5]) /*Optional*/ = ("Vaishnav".to_string(), 20, true, [1, 2, 3, 4, 5]);

  //Slices (Reference to a part of a datatype)
  let slices: &[&str] = &["Hi there", "Bye there"];
}
```

