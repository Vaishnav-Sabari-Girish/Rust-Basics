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
