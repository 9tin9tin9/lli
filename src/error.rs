#[derive(Debug)]
pub enum Error{
    InvalidMemAccess(isize),  // idx: isize
    WriteToNMem(isize),  // idx: isize

    UndefinedVar(String),  // var_name: String
    UnknownOp(String),  // op_name: String

    UnknownLabel(String),  // label_name: String

    WrongTokTypeForOp(&'static str),  // got: &str
    WrongArgType(Vec<&'static str>, &'static str),  // expect: Vec<&str>, got: &str
    WrongArgCount(usize, usize),  // expect: usize, got: usize
    NegativeOrNotInterger(f64),  // got: f64
    NotInterger(f64),  // got: f64

    BadFileDescriptor(std::os::unix::io::RawFd),  // fd tried to open
    IoError(std::io::Error),  // returned from std::io functions
    InvalidOpenOption(u64),  // o_val: u64
}

impl Error {
    // level 0 to silence error msg
    pub fn print(&self, level: usize) {
        if level == 0 {
            return;
        }
        match self {
            Error::InvalidMemAccess(idx) => 
                eprintln!("Invalid memory access: {}", idx),
            Error::WriteToNMem(idx) => 
                eprintln!("Writing to nmem: {}", idx),
            Error::UndefinedVar(var_name) => 
                eprintln!("Undefined variable: {}", var_name),
            Error::UnknownOp(op_name) => 
                eprintln!("Unkwon op: {}", op_name),
            Error::UnknownLabel(label_name) => 
                eprintln!("Unkwon label: {}", label_name),
            Error::WrongTokTypeForOp(got) =>
                eprintln!("Expects Sym, got: {}", got),
            Error::WrongArgType(expect, got) => {
                let mut expect_str = String::from(expect[0]);
                for s in &expect[1..] {
                    expect_str.push_str(" | ");
                    expect_str.push_str(s);
                }
                eprintln!("Expects [{}], got: {}", expect_str, got);
            },
            Error::WrongArgCount(expect, got) => 
                eprintln!("Expects {} args, got: {}", expect, got),
            Error::NegativeOrNotInterger(got) => 
                eprintln!("Expects unsigned integer, got: {}", got),
            Error::NotInterger(got) =>
                eprintln!("Expects an integer, got: {}", got),
            Error::BadFileDescriptor(fd) =>
                eprintln!("Bad file descriptor: {}", fd),
            Error::IoError(e) =>
                eprintln!("IO error: {}", e.to_string()),
            Error::InvalidOpenOption(o) =>
                eprintln!("Invalid open option: {}", o),
        }
    }
}
