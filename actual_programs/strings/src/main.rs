fn main() {
    let mut str_print: String = String::from("Vaishnav");  //Or "Vaishnav".to_string() 
    println!("Original String: {}", str_print);
    str_print.push_str(" Sabari Girish");    
    println!("New string: {}", str_print);

}
