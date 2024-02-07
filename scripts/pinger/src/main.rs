use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ping = 0;

    if args.len() < 2 || args.len() > 2 {
        ping = 5;
    } else {
        if let Ok(v) = args[1].parse::<i32>() {
            ping = v;
        }
    }

    for i in 0..(ping + 1) {
        println!("Ping! {}", i);
    }
}
