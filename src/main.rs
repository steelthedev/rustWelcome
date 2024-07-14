// Compound data types
// Arrays, tuples, slices, and strings (slice strings)




fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Number array:{:?}", numbers);
    
    let fruits: [&str; 5] = ["Apple","bannana","orange","pear","cashew"];

    println!("Fruits Array:{:?}", fruits);
    println!("Fruit Array: {}", fruits[0]);


    // Tuples
    let human: (&str, i32, bool) = ("Alice",30, true);
    println!("{:?}", human);

    let my_mix_tuple: (&str, i32, bool, [i32; 5]) = ("Kratos",23, true,[1,2,3,4,5]);
    println!("Mix Tuple: {:?}", my_mix_tuple);

    // Slices:
    let number_slices: &[i32; 5] = &[1,2,3,4,5];
    println!("My number slices: {:?}", number_slices);


    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("My animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"IT".to_string()];
    println!("My book slices: {:?}", book_slices);

    // String vs string slices(&str)
    // Strings [growable, mutable, owned string type]
    let mut  stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold Says: {}", stone_cold);

    // String Slice (&str)
    let string: String = String::from("Hello, World");
    let slice: &str = &string;
    println!("Slice Value: {}", slice)
    
}

