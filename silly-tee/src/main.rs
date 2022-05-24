use std::io;
use std::process::exit;

enum Ret {
    Incorrect,
    Correct,
    Answer,
}

fn main() {
    let mut buf = String::new();

    let mut ret: Ret = Ret::Incorrect;
    loop {
        let size = io::stdin().read_line(&mut buf);
        match size {
            Ok(0) => {
                break;
            }
            Ok(1) => {}
            Ok(3) => {
                ret = Ret::Answer;
                println!("\n");
            }
            Ok(_) => {
                ret = Ret::Correct;
                eprintln!("{}", buf);
                let bs = vec![buf.trim().as_bytes()];
                for byte_list in bs {
                    for byte in byte_list {
                        print!("{} ", byte);
                    }
                    print!("\n");
                }
            }

            Err(_) => {
                exit(27);
            }
        }
        buf.clear();
    }

    match ret {
        Ret::Correct => exit(0),
        Ret::Incorrect => {
            println!("whoopsie");
            exit(7);
        }
        Ret::Answer => {
            eprintln!("You've found the magic answer");
            println!("FoLl0w 743 w4173 ra8817 n008");
            exit(42)
        }
    }
}
