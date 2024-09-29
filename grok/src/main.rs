use std::time::{SystemTime, UNIX_EPOCH};
use grok::Grok;

fn main() {
    // 打印当前时间戳。单位：秒
    let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("当前时间戳：{}", start);
    let mut i = 0;
    loop {
        cost_time();
        i = i+1;
        if i > 1 {
            break;
        }
    }
    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    println!("当前时间戳：{}", end-start);
}

fn cost_time(){
    // Instantiate Grok
    let mut grok = Grok::default();
    // Add a pattern which might be a regex or an alias
    grok.add_pattern("USERNAME", r"[a-zA-Z0-9._-]+");
    // Compile the definitions into the pattern you want
    let pattern = grok
        .compile("%{USERNAME}", false)
        .expect("Error while compiling!");
    //  Match the compiled pattern against a string
    match pattern.match_against("root") {
        Some(m) => {
            println!("Match found: {:?}", m);
        },
        None => println!("No matches found!"),
    }
}
