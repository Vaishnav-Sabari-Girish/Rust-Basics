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

---

## Stack memory vs Heap memory 

|Property|STACK|HEAP|
|---|---|---|
|**Memory Allocation**|Automatic (allocated/deallocated by the compiler)| Manual (programmer controls allocation/deallocation)|
|**Access Speed**|Faster (due to LIFO structure and locality)|Slower (Due to dynamic allocationand pointer dereferencing)|
|**Size Limit**| Fixed (limited by stack frame)|Dynamic size (limited by system memory)|
|**Data lifetime**| Short lived (scoped to function calls)| Long-lived (persists until explicitely deallocated)|
|**Usage**| Stores local variables , function call data| Stores dynamically sized long-term data (eg: `String` in Rust)| 


---


## String vs String slices (`&str`)

|Property|Strings|String Slices (`&str`)|
|---|---|---|
|**Mutablility**|Mutable| Immutable |
|**Growability**|Growable| Fixed Size|
|**Ownership**|Owned `String` Type| Borrowed reference to `String` data|
|**Memory Allocation**|Allocated on heap| Usually points to data on stack, heap or static memory|
|**Type**| `String` struct| `&str` (reference to a String slice)|
|**Creation**| Created with `String::from()` or `"".to_string()` | Derived from `String` or string literals (eg: "Hello")|
|**Lifetime**|Lives as long as `String` owns the data|Tied to the lifetime of the borrowed data|
|**Storage**|Stores the actual characters (owns the buffer)| Reference to a portion of a string (no ownership)|
|**Performance**| Slow (Due to heap allocation and ownership) | Faster (No allocation, just a reference)|
|**Use case**| When you need to modify or build a String| When you only need to read or view a String|
|**Example**| `let mut s = String::from("hello");` | `let s: &str = "hello";`|

---

## Pointers 

Variables that point to a memory location. 

In Rust, pointers are of 2 types 
1. Safe or references (`&T` and `&mut T`)
2. Unsafe or raw pointers (`*const` or `*mut`)

```rs 
fn main() {
    let mut value = 10;
    
    //Safe Reference 
    let ref_value = &value;
    println!("Value via reference: {}", ref_value);
    
    //Raw pointer (Unsafe)
    let raw_ptr = &mut value as *mut i32;
    unsafe {
        *raw_ptr = 20;   //Modify via rawpointer 
        println!("Value of Raw Pointer: {}", *raw_ptr);
    }
    
    println!("Final value: {}", value);
}
```

---

## Appending to a String 

We will use `<String variable>.push_str(<String to be appended>);`

```rs 
fn main() {
  let mut name: String = String::from("John Doe");   //Or "John Doe".to_string()
  println!("Original String: {}", name);

  name.push_str(" Hi there");
  println!("New String: {}", name);
}
```
