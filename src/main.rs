// #[macro_use]
extern crate lazy_static;
// use std::intrinsics::powf128;

mod acc_soft;
mod database;
mod dsa;
mod math;
mod menu;
mod test;
mod ui;
mod utility;
/* fn main() {
    println!("Hello, world!");
    // powf128(10, 10)
    math::add::add_two(5, 3);
    math::add::add_two(5, 3);
}
    */
use crossterm::event;

fn main() -> std::io::Result<()> {
    // let x = 42;
    // let y = 43;
    // let var1 = &x;
    // let mut var2 = &x;
    // var2 = &y;
    println!("\x1Bc");
    // println!("")
    // utility::func::ident::Js();
    call_to();
    // let (a, b) = call_to();
    // print!("{}, {:?}", a, b.name);
    // print!("{}, {:?}", a, b);
    // test();
    ratatui::run(|terminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("Hello World!", frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}

fn call_to() {
    println!("\n\n\n START OF CALL_TO main.rs");
    // -> (i32, utility::func::ident::Identifier) {
    // let result = math::add::add_two(5, 3);
    // println!("Result: {}", result);
    // let (a, b) = utility::call_to::main();
    // (a, b)
    {
        println!("\n\n\n\n\n\n %%%###### Start of utility::call_to::main() ######%%%%");
        let (answer, id1) = crate::math::c_add::a1();
        println!("Answer: {}", answer);
        println!("Identifier: {}\n", id1.name);
        println!(
            "Calling from main.rs::call_to(), to math::c_add::a1() function, which calls add::add_two() function."
        );
        // eprintln!("error fiel")
        println!("######%%% End of utility::call_to::main() ######%%%%\n\n\n\n\n\n ");
    }
    test();
    // bubble_sort();
    {
        println!("\n\n\n START OF bubble_sort main.rs");
        // let mut arr = vec![2.2, 2.1];

        let (_, mut b) = dsa::c_bubble_sort::bs_1();
        // a =
        b.s_lock("Locked for testing".to_string());
        println!("END OF bubble_sort main.rs\n\n\n\n");
    }
    println!("\n\n\n END OF CALL_TO main.rs");
}
// mod test;
fn test() {
    println!("\n\n\n START OF test main.rs");
    let m = crate::test::m::test();
    println!("Identifier1 = {:?}", m);
    println!("END OF test main.rs\n\n\n\n");
    vec!["walter"];
}
// fn call_to() -> (i32, utility::func::ident::Identifier) {
// let result = math::add::add_two(5, 3);
// println!("Result: {}", result);
// let (a, b) = utility::call_to::main();
// (a, b)
// }
// fn test() {
//     let result = math::add::add_two(5, 3);
//     println!("Result: {}", result);
// }

// fn bubble_sort() {}
