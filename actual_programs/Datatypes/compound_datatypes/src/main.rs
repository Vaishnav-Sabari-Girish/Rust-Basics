fn main() {
    println!("Compound Data Types");
    println!("");

    println!("Arrays");
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];  //[datatype; size]
    println!("Number array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("String array: {:?}", fruits);
    println!("");

    println!("Accessing each array element");
    println!("String array 1st element: {}", fruits[0]);
    println!("String array 2nd element: {}", fruits[1]);
    println!("String array 3rd element: {}", fruits[2]);
    println!("");

    println!("Tuples");
    let human: (String, i32, bool)  /*optional*/ = ("Vaishnav".to_string(), 20, false); 
    println!("Human tuple: {:?}", human);
    println!("");

    println!("Mixed Tuple");
    let mixed_tuple = ("Vaishnav", 20, true, [1, 2, 3, 4, 5]);
    println!("Mixed Tuple: {:?}", mixed_tuple);
    println!("");

    println!("Slices");
    let number_slices: &[i32] = &[1, 2, 3, 4, 5];
    println!("Number slices: {:?}: ", number_slices);

    println!("");

    let pokemon_slices: &[&str] = &["Rayquaza", "Deoxys", "Arceus", "Pikachu"];
    // &str is a String slice which is a reference to a String
    println!("Pokemon Slices: {:?}", pokemon_slices);
    
    // Using reference Strings
    
    let pokemon_regions: &[&String] = &[&"Kanto".to_string(), &"Johto".to_string(), 
    &"Hoenn".to_string(), &"Sinnoh".to_string(), 
    &"Unova".to_string(), &"Kalos".to_string(), &"Alola".to_string(), &"Galar".to_string()];
    
    println!("Pokemon Regions: {:?}", pokemon_regions);

}
