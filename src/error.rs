#[derive(Debug)]
pub enum Error{

    // lexing
    WrongTokTypeForOp(&'static str),  // got
    
    ParseNumError(std::num::ParseFloatError),  // error
    ParseIdxError(std::num::ParseIntError),  // error
    UnterminatedIdx,
    EmptyIdx,
    MissingVarName,
    
    EmptyToken,
    UnexpectedChar(char),
    NonDelimAfterSymEnd(char),
    DoubleQuoteInMiddle,
    UnknownEscapeSequence(char),
    
    // preprocess
    UndefinedVar(String),  // var_name
    UnknownOp(String),  // op_name
    UnknownLabel(String),  // label_name
    
    // runtime 
    InvalidMemAccess(isize),  // idx
    WriteToNMem(isize),  // idx
    WrongArgCount(usize, usize),  // expect, got
    WrongArgType(Vec<&'static str>, &'static str),  // expect, got
    NegativeOrNotInterger(f64),  // got
    NotInterger(f64),  // got
    BadFileDescriptor(std::os::unix::io::RawFd),  // fd tried to open
    IoError(std::io::Error),  // returned from std::io functions
    InvalidOpenOption(u64),  // o_val
}

impl Error {
    // level 0 to silence error msg
    pub fn print(&self, level: usize) {
        if level == 0 {
            return;
        }
        match self {
            Error::WrongTokTypeForOp(got) =>
                eprintln!("Expects Sym, got: {}", got),
            Error::ParseNumError(e) =>
                eprintln!("{}", e.to_string()),
            Error::ParseIdxError(e) =>
                eprintln!("{}", e.to_string()),
            Error::UnterminatedIdx =>
                eprintln!("Unterminated Idx"),
            Error::EmptyIdx =>
                eprintln!("Empty Idx"),
            Error::MissingVarName =>
                eprintln!("Missing Variable Name"),
            
            Error::EmptyToken =>
                eprintln!("Empty Token"),
            Error::UnexpectedChar(c) =>
                eprintln!("Unexpected character: {}", c),
            Error::NonDelimAfterSymEnd(c) =>
                eprintln!("Found non-delimiter after symbol ends: {}", c),
            Error::DoubleQuoteInMiddle =>
                eprintln!("Found unescaped double quote inside string literal"),
            Error::UnknownEscapeSequence(c) =>
                eprintln!("Unknown escape sequence: \\{}", c),

            Error::UndefinedVar(var_name) => 
                eprintln!("Undefined variable: {}", var_name),
            Error::UnknownOp(op_name) => 
                eprintln!("Unkwon op: {}", op_name),
            Error::UnknownLabel(label_name) => 
                eprintln!("Unkwon label: {}", label_name),

            Error::InvalidMemAccess(idx) => 
                eprintln!("Invalid memory access: {}", idx),
            Error::WriteToNMem(idx) => 
                eprintln!("Writing to nmem: {}", idx),
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
