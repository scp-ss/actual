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

check if  itis  posible that i add a mehtod .working_for()
and it bascially modifes the status of idenfiter to working for the fucntion it is in rn, example tui.rs::tui
Or mian.rs or c_v.rs?


and iguess TUI refuses to call to identifers that hav estatus 'working' but since im planing on saving logs
OH fuhh logs adn hwantot

so waht was i thinkig ye, so if it sees that idenfei struct one is locked ,what we do is we need to have another
 thingy or hashmap that has stuats and pid of func, and it is 100% syncehd cauz calling to stautus updates that
 other hasmpa, and if u modiy status hasmp but idenfier is locked it does not allow to modify
 the reaso nfor htis is lets say TUI wants to check if pid-1 is callable, but how will it know if
 accessing identifer is not allowed since it is locked? will simple the hasmpa solution.

add a 'clear screen' / databases program to this.
cler cscrene or chnge color.  or remove option temp.
Or view logs, or all the TUI for calc or a new TUI instance. or refersh. (relaod all databases., notice
how i said all, that means whnever we open file we do so through a util helper . :brian)

I mean i guess thats a littel TOO ambisions for someone who used ai for the TUI, im againsed AI
so ig LEANR rust fiist tsi only been like 2-3moths, leanr the shi wan tt othen TUI then finish.

a text editor would be cool

a  copy result or show desciptin using t could be cool and using arrow key toselect would bekooler;


a vsde extenio ntaht allwo u to copy
        ("1/998001", Rational::from((1, 998001))),
like u wanted to change  the 998001 in 1/... and an extnein did htat ooudl be crazy

*/
fn main() {
    let _ = init_call_to::call();
    // init_call_to::init_all_functions();
    print!("\x1B[2J\x1B[1;1H");
    ui::run_tui();
}
