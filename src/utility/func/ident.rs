#![allow(unused, dead_code)]
#[derive(Clone, Debug)]
pub struct Identifier {
    pub name: String,
    pub id: String,
    pub pid: PID_TABLE,
    pub location: String,
    pub description: Option<String>,
    pub return_type: Option<Vec<String>>,
    pub return_value: Option<Vec<String>>,
    pub args_type: Option<Vec<String>>,
    pub number_of_args: Option<usize>,
    pub args: Option<Vec<Argument>>,
    pub source: Option<String>,
    pub source_call: Option<String>,
    pub cid: Option<String>,
    pub called_by: Option<Vec<String>>,
    pub status: Status,
}
#[derive(Clone, Debug)]
pub struct Status {
    pub status_title: Status_T,
    pub status_code: u32,
}

#[derive(Clone, Debug)]
pub enum Status_T {
    Working(Option<String>),
    Good,
    Failed(Option<String>),
    Paused(Option<String>),
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub name: String,
    pub ty: String,
    pub val: String,
}

static mut PID: u32 = 1;
use std::collections::HashMap;
// unsafe {
// static mut PID_TABLE: HashMap<u32, Identifier> = HashMap::new();
// }
// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() { a } else { b }
// }

fn pid() {
    let v: Vec<i32> = vec![20, 20, 20];
    println!("{:?}", v.clone());
    unsafe {
        PID += 1;
    }
    //-> u32 {
    let pid_table: HashMap<u32, Identifier> = HashMap::new();
    //     let walter = PID_TABLE.get(0);

    //     std::process::id()
}
pub fn test() {
    let w = PID_TABLE.get(0);
    println!("{w:?}");
}
pub fn print_by_pid(n: Option<u32>) {
    match n {
        Some(n) => {
            let m = n; //n as u32;
            let v = get(m);
            print!("{}", m);
            print!("{:?}", v);
        }
        _ => {
            println!("None")
        }
    }
} // wht the hell where u trin a do
pub fn get(pid: u32) -> Option<Identifier> {
    let table = PID_TABLE.TABLE
        .lock()
        .unwrap();

    table
        .get(&pid)
        .cloned()
}
// impl Identifier {
//     fn next_id() -> String {}
//     fn new() {}
//     fn verify(&self) {}
//     fn pid(&self) -> u32 {
//         std::process::id()
//     }
//     fn get_pid_table(&self) -> HashMap<u32, Identifier> {
//         for (pid, id) in &table {
//                 println!("PID: {}, Identifier: {:?}", pid, id);
//         }
//         table
//     }
// }
// use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
        #[derive(Clone, Copy,Debug)]
    pub static ref PID_TABLE_HASH: Mutex<HashMap<u32, Identifier>> = Mutex::new(HashMap::new());
    #[derive(Clone,Debug,Copy)]
    pub static ref PID_TABLE_MAX: u32 = 0;
    #[derive(Clone,Debug)]
    pub static ref PID_TABLE : PID_TABLE1 = PID_TABLE1 {
        TABLE: PID_TABLE_HASH,
        MAX : PID_TABLE_MAX
    };
}
    #[derive(Clone,Debug)]
pub struct PID_TABLE1 {
    TABLE : PID_TABLE_HASH,
    MAX : PID_TABLE_MAX,
}

impl PID_TABLE1 {
    pub fn get(&self, pid: u32) -> Option<Identifier> {
        let table = &self.TABLE.lock().unwrap();

        let nice = table
            .get(&pid)
            .cloned();
        println!("{nice:?}");
        nice
    }
}
impl Identifier {
    pub fn new_pid(self) {
        self.pid  = 
    }
    pub fn add_pid(self) -> Result</*String*/ (), String> {
        let mut table = PID_TABLE1.TABLE
            .lock()
            .unwrap(); // is used if the thingy is poisen, best prevetned if the thread is being pannic before that unuse it

        if table.contains_key(&self.pid) {
            //     return Err(format!("PID {} already exists!", self.pid));
            panic!("PID {} already exists!", self.pid);
        }

        table.insert(self.pid, self.clone());
        println!("ADDED PID {}", self.pid.clone());
        // let nice = "Added".to_string();
        // Result<nice,
        Ok(())
    }
}
