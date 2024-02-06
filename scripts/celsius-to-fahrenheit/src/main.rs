use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        improvise();
    } else {
        let celsius = args[1].parse::<f32>();

        if let Ok(i) = celsius {
            let result = i * 1.80 + 32.00;
            println!("{}C translates to {}F!", args[1], result);
        } else {
            improvise();
        }
    }
}

fn improvise() {
    println!("Ring ring...");
    println!("\"Hello? Europe?\"");
    println!("\"What the hell is a celsius?\"");
    println!("Explanation mumbles in the background");
    println!("\"Eh.. Uhm... The weather is good today! Dont worry! :D\"")
}
