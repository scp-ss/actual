/*

mayee use:
register_fn! {
    name: "guessing_game",
    id: "rb.gg",
    location: "rust_book::guessing_game::guessing_game()",
    description: "So this is the rust book's guessing game chapter.",
    called_by: ["main.rs::main()"],
}
fn guessing_game() -> Identifier { ... }
*/
// pub mod guessing_game {
//     // use std::io::Read;

//     pub fn guessing_game() {
//         println!("Enter Your guess");
//         let mut guess = String::new();
//         loop {
//             guess.clear();
//             match std::io::stdin().read_line(&mut guess) {
//                 Ok(num) => {
//                     println!("You entered {num}");
//                     match guess
//                         .trim()
//                         .parse::<i32>()
//                     {
//                         Ok(parsed) => {
//                             println!("You entered {parsed}");
//                             break;
//                         }
//                         Err(err) => {
//                             eprintln!("Failed to convert to integer, {err}");
//                         }
//                     }
//                 }
//                 Err(err) => {
//                     eprintln!("{err}")
//                 }
//             }
//         }
//         println!("guess = {guess}");
//     }
// }

pub use guessing_game::guessing_game;
// here it is
pub mod guessing_game {
    use crate::utils::{
        func::ident::{Identifier, Status},
        get_input,
    };
    use rand::prelude::*;
    // also in the init folder in all functiosn that take input create a systme so that the inputs are auto fed.
    //and for funcs like this where they input has to equal to smt or lets say it lawasy has to be pstile
    // createa  input value = Enum::Type(val), and each func identifer has type, and if type type not found panic, if found.
    // use that type in example of inputs (ill adda 'two input takaen' and' input type'thing),
    // so that this loading process is insatn, and make a loading animaton
    // use rug::rand;
    pub fn guessing_game() -> Identifier {
        println!("\n\n\n\n\n\n ###### Start of rust_book::guessing_game::guessing_game() ######"); //lwk i think this gotta go.
        // lets add anotehr mutex or global thingy that holds the location
        // as a  string.
        // then we can use it to print his cauz this shit is ugly af to write again and agian.
        // ye ig anotehr todo list
        let mut rng = rand::rng();
        let random: u32 = rng.random_range(0..=100);
        println!("random : {random}");
        'matchi: loop {
            let guess: u32 = get_input("Enter guess ");
            match guess.cmp(&random) {
                std::cmp::Ordering::Less => println!("Number is too smol"),
                std::cmp::Ordering::Greater => println!("Numbr lwk big"),
                std::cmp::Ordering::Equal => {
                    println!("equal gng");
                    break 'matchi;
                }
            }
        }
        //#####################################################################
        //#####################################################################
        //#####################################################################
        //#####################################################################
        let mut id1 = Identifier {
            name: "guessing_game".to_string(),
            id: "rb.gg".to_string(),
            pid: None,
            location: "rust_book::guessing_game::guessing_game()".to_string(),
            description: Some(
                "So this is the rust book's guessingame chapter. i will add more".to_string(),
            ),
            return_type: None,
            return_value: None,
            args_type: None,
            number_of_args: None,
            args: None,
            source: Some(
                "rust_book::guessing_game::guessing_game()::guessing_game(){}".to_string(),
            ),
            source_call: Some("call.rb.gg".to_string()),
            cid: None,
            called_by: Some(vec!["main.rs::main()".to_string()]),
            status: Status {
                status_title: crate::utils::func::ident::Status_T::Working(Some(
                    "rust_book".to_string(),
                )),
                status_code: 523,
            },
            validate: false,
            func_pointer: Some(guessing_game as fn() -> Identifier),
        };
        id1.print_s();
        id1.generate_pid()
            .validate();
        id1.print_s();
        let pid = id1
            .pid
            .expect("NO PID AHAH");
        println!("id1 is {}", id1.validate.clone());
        println!("PID = {}, {:?}", std::process::id(), pid);
        println!("Calling from rust_book::guessing_game::guessing_game()::guessing_game");
        println!("###### End of rust_book::guessing_game::guessing_game() ######\n\n\n\n\n");
        println!("{:?}", id1.clone());
        id1.s_status(Some("Free to be used".to_string()), Some(200));
        id1.print_s();

        id1
    }
}
