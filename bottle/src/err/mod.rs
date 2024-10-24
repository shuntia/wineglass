use std::fs;

pub struct Warning {
    pub code: i32,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub struct Info {
    pub code: i32,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub struct Error {
    pub code: i32,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub struct Fatal {
    pub code: i32,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub struct InternalWarning {
    pub code: i64,
    pub file: String,
    pub message: String,
}

pub struct InternalInfo {
    pub code: i64,
    pub file: String,
    pub message: String,
}

pub struct InternalError {
    pub code: i64,
    pub file: String,
    pub message: String,
}

pub struct InternalFatal {
    pub code: i64,
    pub file: String,
    pub message: String,
}
pub enum Diagnostic {
    ///container for Diagnostics of code
    Warning(Warning),
    Info(Info),
    Error(Error),
    Fatal(Fatal),
}

pub enum InternalReport {
    ///Errors produced by the compiler. These are unexpected errors.
    InternalWarning(InternalWarning),
    InternalInfo(InternalInfo),
    InternalError(InternalError),
    InternalFatal(InternalFatal),
}

pub enum Response<T> {
    Ok(T),
    Report(InternalReport),
}

impl Diagnostic {
    pub fn get_code(&self) -> i32 {
        match self {
            Diagnostic::Warning(w) => w.code,
            Diagnostic::Info(i) => i.code,
            Diagnostic::Error(e) => e.code,
            Diagnostic::Fatal(f) => f.code,
        }
    }
    pub fn get_message(&self) -> String {
        match self {
            Diagnostic::Warning(w) => &w.message,
            Diagnostic::Info(i) => &i.message,
            Diagnostic::Error(e) => &e.message,
            Diagnostic::Fatal(f) => &f.message,
        }
        .to_owned()
    }
    pub fn get_file(&self) -> String {
        match self {
            Diagnostic::Warning(w) => &w.file,
            Diagnostic::Info(i) => &i.file,
            Diagnostic::Error(e) => &e.file,
            Diagnostic::Fatal(f) => &f.file,
        }
        .to_owned()
    }
    pub fn get_location(&self) -> (usize, usize) {
        match self {
            Diagnostic::Warning(w) => (w.loc.0, w.loc.1),
            Diagnostic::Info(i) => (i.loc.0, i.loc.1),
            Diagnostic::Error(e) => (e.loc.0, e.loc.1),
            Diagnostic::Fatal(f) => (f.loc.0, f.loc.1),
        }
    }
    pub fn get_length(&self) -> usize {
        match self {
            Diagnostic::Warning(w) => w.len,
            Diagnostic::Info(i) => i.len,
            Diagnostic::Error(e) => e.len,
            Diagnostic::Fatal(f) => f.len,
        }
    }
    pub fn get_idx(&self) -> (usize, usize, usize) {
        match &self {
            Diagnostic::Warning(w) => (w.loc.0, w.loc.1, w.len),
            Diagnostic::Info(i) => (i.loc.0, i.loc.1, i.len),
            Diagnostic::Error(e) => (e.loc.0, e.loc.1, e.len),
            Diagnostic::Fatal(f) => (f.loc.0, f.loc.1, f.len),
        }
    }
    pub fn get_problem(&self) -> String {
        match &self {
            Diagnostic::Warning(w) => {
                if w.code == 0 {
                    return "-".to_owned();
                }
            }
            Diagnostic::Info(i) => {
                if i.code == 0 {
                    return "-".to_owned();
                }
            }
            Diagnostic::Error(e) => {
                if e.code == 0 {
                    return "-".to_owned();
                }
            }
            Diagnostic::Fatal(f) => {
                if f.code == 0 {
                    return "-".to_owned();
                }
            }
        }
        let binding = fs::read_to_string(match &self {
            Diagnostic::Warning(w) => &w.file,
            Diagnostic::Info(i) => &i.file,
            Diagnostic::Error(e) => &e.file,
            Diagnostic::Fatal(f) => &f.file,
        })
        .expect("Invalid Diagnostic! Failed to read to string!");
        let loc_idx = &self.get_idx();
        let spliced_str = match binding.lines().nth(loc_idx.0) {
            Some(line) => line,
            None => panic!("Invalid Diagnostic! Failed to find line"),
        };
        spliced_str[loc_idx.1..loc_idx.2].to_owned()
    }
    pub fn display(&self) -> String {
        let idx = self.get_idx();
        let problem = self.get_problem();
        let message = self.get_message();
        let code = self.get_code();
        let mut display: String = code.to_string();
        display.push('\n');
        display.push_str(&format!("at ({}, {}),\n", idx.0, idx.1));
        display.push_str(&format!("{}\n", problem));
        display.push_str(&(" ".repeat(idx.1)));
        display.push('^');
        display.push_str(&("~".repeat(idx.2)));
        display.push('\n');
        display.push_str(&message);
        display
    }
}

impl InternalReport {
    pub fn get_type(&self) -> &str {
        match self {
            Self::InternalWarning(_) => "Warning",
            Self::InternalError(_) => "Error",
            Self::InternalFatal(_) => "Fatal",
            Self::InternalInfo(_) => "Info",
        }
    }
    pub fn get_code(&self) -> i64 {
        match self {
            InternalReport::InternalWarning(w) => w.code,
            InternalReport::InternalInfo(i) => i.code,
            InternalReport::InternalError(e) => e.code,
            InternalReport::InternalFatal(f) => f.code,
        }
    }
    pub fn get_message(&self) -> String {
        match self {
            InternalReport::InternalWarning(w) => &w.message,
            InternalReport::InternalInfo(i) => &i.message,
            InternalReport::InternalError(e) => &e.message,
            InternalReport::InternalFatal(f) => &f.message,
        }
        .to_owned()
    }
    pub fn get_file(&self) -> String {
        match self {
            InternalReport::InternalWarning(w) => &w.file,
            InternalReport::InternalInfo(i) => &i.file,
            InternalReport::InternalError(e) => &e.file,
            InternalReport::InternalFatal(f) => &f.file,
        }
        .to_owned()
    }
    pub fn display(&self) -> String {
        let message = self.get_message();
        let code = self.get_code();
        let t = self.get_type();
        let mut display: String = format!("{:?}{:?} ", t, code);
        display.push_str(&message);
        display
    }
    pub fn expect(&self, m: &str) {
        match self {
            InternalReport::InternalError(e) => panic!("{}\nError! {}", m, e.message),
            InternalReport::InternalFatal(f) => panic!("{}\nFatal! {}", m, f.message),
            _ => {}
        }
    }
    pub fn unwrap(&self) {
        match self {
            InternalReport::InternalError(e) => panic!("Error! {}", e.message),
            InternalReport::InternalFatal(f) => panic!("Fatal! {}", f.message),
            _ => {}
        }
    }
}
