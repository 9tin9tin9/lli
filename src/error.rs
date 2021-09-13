#[derive(Debug)]
pub enum Error{
    InvalidMemAccess(isize),  // idx: isize
    WriteToNMem(isize),  // idx: isize

    UndefinedVar(String),  // var_name: String
    UnknownOp(String),  // op_name: String

    UnknownLabel(String),  // label_name: String
    CorruptedLabel(String, usize),  // labe_name: String, idx: usize

    WrongTokTypeForOp(&'static str),  // got: &str
    WrongArgType(Vec<&'static str>, &'static str),  // expect: Vec<&str>, got: &str
    WrongArgCount(usize, usize),  // expect: usize, got: usize
    NegativeOrNotInterger(f64),  // got: f64
    NotInterger(f64),  // got: f64

    InvalidOutType(String),  // got: String

    BadFileDescriptor(std::os::unix::io::RawFd),  // fd tried to open
    IoError(std::io::Error),  // returned from std::io functions
    InvalidOpenOption(u64),  // o_val: u64
}
