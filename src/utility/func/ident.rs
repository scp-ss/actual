#![allow(unused, dead_code)]
use std::collections::HashMap;
// use std::sync::Mutex;
use std::sync::{
    LazyLock, Mutex,
    atomic::{AtomicU64, Ordering},
};
#[derive(Clone, Debug)]
pub struct Identifier {
    pub name: String,
    pub id: String,
    pub pid: Option<u64>,
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
    pub validate: bool,
}
#[derive(Clone, Debug)]
pub struct Status {
    pub status_title: Status_T,
    pub status_code: u64,
}

#[derive(Clone, Debug)]
#[allow(non_camel_case_types)]
pub enum Status_T {
    Working(Option<String>),
    Good(Option<String>),
    Failed(Option<String>),
    Paused(Option<String>),
}

#[derive(Clone, Debug)]
pub struct Argument {
    pub name: String,
    pub ty: String,
    pub val: String,
}

impl Identifier {
    pub fn print_s(&self) {
        println!("{:?}", self.status);
    }
    pub fn s_status(&mut self, a: Option<String>, code: Option<u64>) {
        self.status
            .status_code = code.unwrap_or(41414141);

        self.status
            .status_title = match self
            .status
            .status_code
        {
            100..=199 => Status_T::Working(a),
            200..=299 => Status_T::Good(a),
            400..=599 => Status_T::Failed(a),
            _ => Status_T::Failed(None),
        };
    }
    pub fn generate_pid(&mut self) -> &mut Identifier {
        if self.pid.is_none() {
            self.pid = Some(PID_TABLE.next_pid());
        }
        self
    }
    pub fn s_lock(&mut self) {}
    // pub fn generate_pid(&mut self) -> &mut Identifier {
    //     // dont do self her cauz if we do sel id would be consued, adn if we do u32 then method vlalidate wont get called
    //     self.pid = PID_TABLE.next_pid();
    //     self
    // }
    pub fn validate(&mut self) -> bool {
        if self.pid.is_none() {
            self.generate_pid();
        }
        let pid = self.pid.unwrap(); // safe: guaranteed Some by the line above

        let mut table = PID_TABLE
            .table
            .lock()
            .unwrap();
        match table.get(&pid) {
            Some(_) => {
                println!("validation => false (duplicate PID: {pid})");
                false
            }
            None => {
                self.validate = true;
                table.insert(pid, self.clone());
                true
            }
        }
    }
    // pub fn validate(&mut self) -> bool {
    //     // if self.validate =
    //     let mut table = PID_TABLE
    //         .table
    //         .lock()
    //         .unwrap();
    //     // PID_TABLE
    //     match table.get(&self.pid) {
    //         Some(_) => {
    //             println!("validation => false (duplicate PID: {})", self.pid);
    //             return false;
    //         }
    //         None => {
    //             self.validate = true;
    //             table.insert(self.pid, self.clone());
    //             return true;
    //         }
    //     }
    //     table
    //         .get(&self.pid)
    //         .is_some()

    //     // .lock()
    // }
    // fn validate(&self) {}
}
pub static PID_TABLE: LazyLock<PidTable> = LazyLock::new(|| PidTable {
    table: Mutex::new(HashMap::new()),
    next: AtomicU64::new(0), // first call returns 0, next call returns 1, etc.
});

pub struct PidTable {
    table: Mutex<HashMap<u64, Identifier>>,
    next: AtomicU64,
}
impl PidTable {
    pub fn print(&self, pid: Option<u64>) {
        let table = &self
            .table
            .lock()
            .unwrap();
        match pid {
            Some(target) => match table.get(&target) {
                Some(identifier) => println!("{target} -> {identifier:?}"),
                None => println!("no identifier found for pid {target}"),
            },

            None => {
                for (pid, identifier) in table.iter() {
                    println!("{pid} -> {identifier:?}")
                }
            }
        }
    }
    pub fn next_pid(&self) -> u64 {
        self.next
            .fetch_add(1, Ordering::SeqCst)
    }

    // fn generate_pid(){

    // AtomicU64::SeSeqCs
    // }
    // fn print_table()
    // fn print_pid()
}

// impl Identifier {
//     pub fn new_pid(self) {
//         self.pid  =
//     }
//     pub fn add_pid(self) -> Result</*String*/ (), String> {
//         let mut table = PID_TABLE1.TABLE
//             .lock()
//             .unwrap(); // is used if the thingy is poisen, best prevetned if the thread is being pannic before that unuse it

//         if table.contains_key(&self.pid) {
//             //     return Err(format!("PID {} already exists!", self.pid));
//             panic!("PID {} already exists!", self.pid);
//         }

//         table.insert(self.pid, self.clone());
//         println!("ADDED PID {}", self.pid.clone());
//         // let nice = "Added".to_string();
//         // Result<nice,
//         Ok(())
//     }
// }

// static mut PID: u32 = 1;
// unsafe {
// static mut PID_TABLE: HashMap<u32, Identifier> = HashMap::new();
// }
// fn longest(a: &str, b: &str) -> &str {
//     if a.len() > b.len() { a } else { b }
// }

// fn pid() {
//     let v: Vec<i32> = vec![20, 20, 20];
//     println!("{:?}", v.clone());
//     unsafe {
//         PID += 1;
//     }
//     //-> u32 {
//     let pid_table: HashMap<u32, Identifier> = HashMap::new();
//     //     let walter = PID_TABLE.get(0);

//     //     std::process::id()
// }
// pub fn test() {
//     let w = PID_TABLE.get(0);
//     println!("{w:?}");
// }
// pub fn print_by_pid(n: Option<u32>) {
//     match n {
//         Some(n) => {
//             let m = n; //n as u32;
//             let v = get(m);
//             print!("{}", m);
//             print!("{:?}", v);
//         }
//         _ => {
//             println!("None")
//         }
//     }
// } // wht the hell where u trin a do
// pub fn get(pid: u32) -> Option<Identifier> {
//     let table = PID_TABLE.TABLE
//         .lock()
//         .unwrap();

//     table
//         .get(&pid)
//         .cloned()
// }
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

// lazy_static! {
//         #[derive(Clone, Copy,Debug)]
//     pub static ref PID_TABLE_HASH: Mutex<HashMap<u32, Identifier>> = Mutex::new(HashMap::new());
//     #[derive(Clone,Debug,Copy)]
//     pub static ref PID_TABLE_MAX: u32 = 0;
//     #[derive(Clone,Debug)]
//     pub static ref PID_TABLE : PID_TABLE1 = PID_TABLE1 {
//         TABLE: PID_TABLE_HASH,
//         MAX : PID_TABLE_MAX
//     };
// }
//     #[derive(Clone,Debug)]
// pub struct PID_TABLE1 {
//     TABLE : PID_TABLE_HASH,
//     MAX : PID_TABLE_MAX,
// }

// impl PID_TABLE1 {
//     pub fn get(&self, pid: u32) -> Option<Identifier> {
//         let table = &self.TABLE.lock().unwrap();

//         let nice = table
//             .get(&pid)
//             .cloned();
//         println!("{nice:?}");
//         nice
//     }
// }
