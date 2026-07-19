pub mod guessing_game {
    // use std::io::Read;

    pub fn guessing_game() {
        println!("Enter Your guess");
        let mut guess = String::new();
        loop {
            guess.clear();
            match std::io::stdin().read_line(&mut guess) {
                Ok(num) => {
                    println!("You entered {num}");
                    match guess
                        .trim()
                        .parse::<i32>()
                    {
                        Ok(parsed) => {
                            println!("You entered {parsed}");
                            break;
                        }
                        Err(err) => {
                            eprintln!("Failed to convert to integer, {err}");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("{err}")
                }
            }
        }
    }
}

pub use guessing_game::guessing_game;
// here it is
