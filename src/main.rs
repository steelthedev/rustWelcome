
fn main() {
  hello_world();
  tell_height(45);
  human_id("Iyanuoluwa", 26, 67.0);

  let x = {
    let price: i32 = 50;
    let qty: i32 = 2;
    price * qty

  };
  println!("Result is: {}", x);

  let y: i32 = add(5, 10);
  println!("Value of y is: {}", y);

      // Calling BMI func
      let weight: f64 = 50.0;
      let height: f64 = 40.0;
      let bmi: f64 = calculate_bmi(weight, height);
      println!("BMI value is {:.2}", bmi);
    
}

fn calculate_bmi(weight_kg:f64, height_m:f64) -> f64{
    weight_kg/(height_m * height_m)
}


fn hello_world(){
    println!("Hello, Rust")
}

// Insert parameters

fn tell_height(height: i32){
    println!("My height is {} cm", height)
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}, I am {} years old, and my height is {} cm",name, age, height);
}

fn add(a:i32, b:i32) -> i32{
    a+b
}



// Expressions and statements
// Expression: Anything that returns a value(
// 5
// true & false
// if conditions 
// add(4,5) 
//)

// Statement: Anything that does not return a value


