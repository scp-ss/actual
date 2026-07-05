use crate::utility::{
    self,
    func::ident::{Argument, Identifier, PID_TABLE, Status, Status_T},
    r#use::get_input,
};
pub fn a1() -> (i32, Identifier) {
    // cutility::
    println!("\n\n\n\n\n\n ###### Start of math::c_add::add_two() ######");
    let a: i32 = get_input("Enter First Number ");
    let b: i32 = get_input("Enter Second Number ");
    let result = crate::math::add::add_two(a, b);
    let mut id1 = Identifier {
        name: "add_two".to_string(),
        id: "madd.1".to_string(),
        pid: 0,
        location: "math::add".to_string(),
        description: Some("Adds two integers and returns the result.".to_string()),
        return_type: Some(vec!["i32".to_string()]),
        return_value: Some(vec![result.to_string()]),
        args_type: Some(vec!["i32".to_string(), "i32".to_string()]),
        number_of_args: Some(2),
        args: Some(vec![
            Argument {
                name: "a".to_string(),
                ty: "i32".to_string(),
                val: "5".to_string(),
            },
            Argument {
                name: "b".to_string(),
                ty: "i32".to_string(),
                val: "3".to_string(),
            },
        ]),
        source: Some("math::c_add::a(){}".to_string()),
        source_call: Some("call.madd.1".to_string()),
        cid: Some("calle.1".to_string()),
        called_by: Some(vec!["main.rs::main()".to_string()]),
        status: Status {
            status_title: utility::func::ident::Status_T::Working(Some("for_c_math".to_string())),
            status_code: 523,
        },
    };
    id1.clone()
        .add()
        .unwrap_or_else(|e| eprintln!("Could not add; \n{e}"));
    // .expect("FAILED_TO_ADD PID {}");
    println!("Result: {}", result);
    println!("PID = {}, {}", std::process::id(), id1.pid);
    // println!("")
    // let w = PID_TABLE.get(0); // works
    // println!("Walter {:?}", w);
    println!("###### End of math::c_add::add_two() ######\n\n\n\n\n");
    id1.status
        .status_title = utility::func::ident::Status_T::Good;
    (result, id1)
}
