fn main() {
    // Approach 1

    let result = divide(10.0, 2.0);
    match result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by Zero!")
    }

    // Approach 2

    let result2 = sub(1.0, 2.0);
    match result2 {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("{err}")
    }
   

}


fn divide(num: f64, den: f64) -> Option<f64>{
    if den == 0.0{
        None
    }else {
        Some(num/den)
    }
    
}


fn sub(num1: f64, num2: f64) -> Result<f64, String>{
    if num1 < num2{
        Err("Number 1 cannot be smaller".to_string())
    }else {
        Ok(num1-num2)
    }
    
}

