use std::panic;

use scicalc_rs::parser::eval;
fn main() {
    panic::set_hook(Box::new(|_info| {
        // do nothing
        //TODO: Add proper error handling
    }));
    let result = panic::catch_unwind(|| {
        eval("(1.0 ± 0.1) * (3.0 ± 0.1)")
    });
    match result {
        Ok(res) => println!("{}", res),
        Err(_) => println!("ERROR"),
    }
}
