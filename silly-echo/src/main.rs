use std::env;
use std::process::exit;

fn main() {
    let response = "no";
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You're not doing it right.");

        exit(86);
    }

    let first = &*args[1];

    if first != "69" && first.parse::<f64>().is_ok() {
        println!("I don't wanna.");
        exit(1);
    }

    match &*args[1] {
        "69" => {
            if args.len() < 3 {
                println!("ha ha nice");
            } else {
                println!("{} -- nice", args[2]);
            }
            exit(66)
        }
        "no" => {
            println!("yes");
            exit(1);
        }
        _ => {
            println!("{}", response);
            exit(0)
        }
    }
}
