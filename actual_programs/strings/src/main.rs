fn main() {
    let mut str_print: String = String::from("Vaishnav");  //Or "Vaishnav".to_string() 
    println!("Original String: {}", str_print);
    str_print.push_str(" Sabari Girish");    
    println!("New string: {}", str_print);

    println!("");

    println!("String Slices");
    let word: String = String::from("Hello World");
    let slice_full: &str = &word;
    let slice_segmented: &str = &word[0..5];

    println!("Sting slice (full) : {}", slice_full);
    println!("String Slice (segmented): {}", slice_segmented);

}
