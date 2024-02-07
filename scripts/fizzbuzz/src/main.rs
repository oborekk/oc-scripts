use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        println!("Program Frizzed Buzzed! Wrong input!");
    } else {
        let num = args[1].parse::<i32>();

        if let Ok(v) = num {
            for i in 1..(v + 1) {
                if i % 5 == 0 && i % 3 == 0 {
                    println!("FizzBuzz");
                } else if i % 5 == 0 {
                    println!("Buzz");
                } else if i % 3 == 0 {
                    println!("Fizz");
                } else {
                    println!("{i}");
                }
            }
        }
    }
}
