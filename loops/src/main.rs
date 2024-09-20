fn main() {
    for number in (1..4).rev() {
        println!("number is: {}", number);
    }

    for number in 1..4 {
        println!("number is: {}", number);
    }

    let a = [1,2,3,4,5,6];
    let mut index = 0;
    while index < 5 {
        println!("index is: {}", index);
        index += 1;
    }

    for element in a {
        println!("The value of element is: {}", element);
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -=1;
    }
    println!("LIFTOFF!!!");
}
