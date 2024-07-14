fn main() {
    
    // //Loop keyword (Unconditional loop; runs till you tell it to stop)
    // let mut _x = 0;
    // let result = loop {
    //      _x += 1;

    //     if _x > 5{
    //         break _x * 2;
    //     }
    // };

    // println!("{}", result);

    // Loop labels

    // let mut counter = 0;

    // 'counting_up: loop {
    //     println!("count = {counter}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9{
    //             break;
    //         }
    //         if counter == 2{
    //             break 'counting_up;
    //         }

    //         remaining -= 1
    //     }
    //     counter += 1
    // }


    // While loop

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
        break;
    }

    // For Loop

    let a = [1,2,3,4,5,6];
    let b = ["a","b","c","d"];

    for element in a  {
        println!("{element}");
    }

    for element in b  {
        println!("{element}");
    }
}