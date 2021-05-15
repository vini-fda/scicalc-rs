use std::{env, panic};

use scicalc_rs::parser::eval;

fn show_usage() {
    println!("Usage: scicalc-rs [expression]");
}
fn main() {
    panic::set_hook(Box::new(|_info| {
        // do nothing
        //TODO: Add proper error handling
    }));

    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        println!("Error: Not enough args.");
        show_usage();
    } else if args.len() == 2 {
        let result = panic::catch_unwind(|| {
            let text = args[1].as_str();
            eval(text)
        });
        match result {
            Ok(res) => println!("{}", res),
            Err(_) => println!("Error: could not parse expression."),
        }
    } else {
        println!("Error: too many args.");
        show_usage();
    }
}
