use crate::utility::r#use::get_input;
use crate::utility::{
    self,
    func::ident::{Argument, Identifier, /*PID_TABLE,*/ Status /*Status_T*/},
    // r#use::get_input,
};
pub fn test() {
    let mut simga = Walter { test: 0 };
    simga
        .add(get_input("Enter A number"))
        .print();
    let mut id1 = Identifier {
        name: "m".to_string(),
        id: "testm1".to_string(),
        pid: None,
        location: "test::m::test()".to_string(),
        description: Some("Adds two integers and returns the result.".to_string()),
        return_type: Some(vec!["i32".to_string()]),
        return_value: Some(vec![
            simga
                .test
                .to_string(),
        ]),
        args_type: None,
        number_of_args: None,
        args: None,
        source: Some("test::m::test(){}".to_string()),
        source_call: Some("mtest.m".to_string()),
        cid: Some("testing1".to_string()),
        called_by: Some(vec!["main.rs::test()".to_string()]),
        status: Status {
            status_title: utility::func::ident::Status_T::Working(Some("for_c_math".to_string())),
            status_code: 523,
        },
        validate: false,
    };
    id1.print_s();
    id1.generate_pid()
        .validate();
    id1.print_s();
    // println!("")
    id1.s_status(Some("Free to be used".to_string()), Some(200));
}

// I want to see what happenes if I derefence a value.
pub struct Walter {
    test: u32,
}
impl Walter {
    fn add(&mut self, b: u32) -> &Walter {
        self.test += b;
        // *self
        // self.test
        &*self
        // ok wnated to check this. good to know it not psosboe
    }
    fn print(&self) {
        println!("{}", self.test);
    }
}
