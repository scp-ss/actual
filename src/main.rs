#[macro_use]
extern crate lazy_static;
// use std::intrinsics::powf128;

mod acc_soft;
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
    println!("\x1Bc");
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
}
// mod test;
fn test() {
    crate::test::m::test();
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
