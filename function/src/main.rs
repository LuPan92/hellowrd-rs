fn main() {
    println!("Hello, world!");
    func1();
    func2(1);
    func3(1,'h');
    // 表达式
    let y = {
        let x = 3;
        x + 1
    };
    println!("y = {}", y);
    let x = func4(5);
    println!("x = {}", x);
    
    func5(20);

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number = {}", number)
    }

fn func1(){
    println!("func1");
}

fn func2(x: i32){
    println!("func2 {}", x);
}

fn func3(x: i32,unit_label: char){
    println!("func2 {} {}", x,unit_label);
}

fn func4(x: i32) -> i32 {
    x * 5
}

fn func5(x: i32) {
    if x % 5 == 0 {
        println!("{} is divisible by 5", x);
    } else if x % 6 == 0 {
        println!("{} is not divisible by 6", x);
    }else {
        println!("{} is neither divisible by 5 nor by 6", x);
    }
}
