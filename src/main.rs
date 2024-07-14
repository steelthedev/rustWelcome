
fn main() {
//  let age: u16 = 18;

//  if age >= 18{
//     println!("You are old enough to code");
//  }else {
//      println!("You can't code");
//  }
 
// Multiple conditions with else if
 let number: i32 = 100;

 if number % 4 == 0{
    println!("Number is divisible by 4");
 }else if number % 3 == 0 {
    println!("Number is divisible by 3");
 }else if number % 2 == 0 {
    println!("Number is divisible by 2");
 }else {
     println!("Number not divisible by 4, 3, or 2");
 }

 // If in a let statement

 let condition = true;
 let number = if condition {5} else {6};
 println!("{}", number)
}