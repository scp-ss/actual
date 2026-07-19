pub mod guessing_game {
    pub fn guessing_game() {
        println!("Enter Your guess");
        let mut guess = String::new();
        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take in guess");
    }
}

pub use guessing_game::guessing_game;
