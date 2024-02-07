use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.len() > 2 {
        improvise();
    } else {
        let fahrenheit = args[1].parse::<f32>();

        if let Ok(i) = fahrenheit {
            let result = (i - 32.00) * 5.00 / 9.00;
            println!("{}F translates to {}C!", args[1], result);
        } else {
            improvise();
        }
    }
}

fn improvise() {
    println!("Ring ring...");
    println!("\"Hallo? American?\"");
    println!("\"Fahrenh what?\"");
    println!("Explanation mumbles in the background");
    println!("\"Ja.. jaa...\"")
}
