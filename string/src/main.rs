fn main() {
    let s = String::from("hello");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // 分配在堆上不会拷贝
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}, world!", s1); // 报错

    let s1 = gives_ownership(); 

    let (s2,len) = calculate_length(s1);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn calculate_length(s: String) -> (s,usize) {
    let len = s.len();
    (s,len)
}

