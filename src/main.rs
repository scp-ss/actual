// #[macro_use]
extern crate lazy_static;
// use std::vec;
#[allow(unused)]
pub(crate) use utils::macros::m_ident::c_wrapper;
// use vector_double::DoubleAll;

// use std::intrinsics::powf128;
/*
make a prelude for each:

// prelude.rs (crate root)
pub use crate::utils::prelude::*;
pub use crate::dsa::prelude::*;
pub use crate::math::prelude::*;
 */
pub mod acc_soft;
mod database;
pub mod dsa;
pub mod init_call_to;
pub mod math;
pub mod menu;
pub mod prelude;
pub mod reexports;
pub mod rust_book;
pub mod test;
pub mod traits;
pub mod tui;
pub mod ui;
pub mod utils;
/* fn main() {
    println!("Hello, world!");
    // powf128(10, 10)
    math::add::add_two(5, 3);
    math::add::add_two(5, 3);
}
    */
// use crossterm::event;
// use utils::func::init_call_to;

// mod utils;

// use ui::tui;
// use utils::func::init_call_to;
/*
A couple rpoblesm other then the fact that we want custom ye,
well we mark funcs with 'input takes' as smt else, and lockthingy,
adn thnebfoer calling to func we chkec its id1 adn seei fi t CAN be caleld.


arg types nad value type as a Int, string or wtvr



anoteh thing registry, TUI.rs , and uhh anyhgn related to registry is
NOT coded by u, MAKE srue to leanr them after learnjng basics of rust
 */
fn main() {
    let _ = init_call_to::call();
    // init_call_to::init_all_functions();
    print!("\x1B[2J\x1B[1;1H");
    ui::run_tui();
}
