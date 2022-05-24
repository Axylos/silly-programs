use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => {
            eprintln!("dunno what to do");
            exit(1);
        }
        2 => {
            let arg = &*args[1];
            let n = arg.parse::<i32>();
            if n.is_ok() {
                print_seq(n.unwrap());
            } else {
                eprintln!("maybe try a number");
                exit(4);
            }
        }
        _ => {
            println!("ouch.  these are not the droids you are looking for");
            exit(2);
        }
    }
}

fn print_seq(n: i32) {
    let speech = "In any war, there are calms between the storms. There will be days when we lose faith. Days when our allies turn against us. But the day will never come when we forsake this planet and its people. For I am Optimus Prime, and I send this message to the universe: We are here. We are home.";

    let range = std::ops::Range { start: 0, end: n };

    let parts = speech.split(" ").collect::<Vec<&str>>();
    let size: i32 = parts.len() as i32;
    for i in range {
        let idx: i32 = i % size;
        if idx == 0 {
            print!("\n\n");
        }
        print!("{} ", parts[idx as usize]);
    }
}
