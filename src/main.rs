use serde_json::{Error, Result, Value};
use std::env;

fn pretty_print(json_str: &str) -> Result<()> {
    let v: Value = serde_json::from_str(json_str)?;

    let pretty_json = serde_json::to_string_pretty(&v)?;
    println!("{}", pretty_json);

    Ok(())
}

fn print_error_location(json_str: &str, error: &Error) {
    let one_based_col = error.column();
    if one_based_col > 0 {
        let zero_based_col = one_based_col - 1;
        println!("{}", json_str);
        println!("{:>1$}^", "", zero_based_col);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} json_string", args[0]);
        std::process::exit(1);
    }

    if let Err(e) = pretty_print(&args[1]) {
        println!("Error parsing JSON: {}", e);
        print_error_location(&args[1], &e);
        std::process::exit(1);
    }
}
