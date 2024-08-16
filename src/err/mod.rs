use std::fs;


pub struct Warning {
    pub code:i64,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub struct Info {
    pub code:i64,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub struct Error {
    pub code:i64,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub struct Fatal {
    pub code:i64,
    pub file: String,
    pub message: String,
    pub loc: (usize, usize),
    pub len: usize,
}

pub enum Diagnostic {
    Warning(Warning),
    Info(Info),
    Error(Error),
    Fatal(Fatal),
}

impl Diagnostic {
    pub fn get_code(&self) -> i64 {
        match self {
            Diagnostic::Warning(w) => w.code,
            Diagnostic::Info(i) => i.code,
            Diagnostic::Error(e) => e.code,
            Diagnostic::Fatal(f) => f.code,
        }
    }
    pub fn get_message(&self) -> &str {
        match self {
            Diagnostic::Warning(w) => &w.message,
            Diagnostic::Info(i) => &i.message,
            Diagnostic::Error(e) => &e.message,
            Diagnostic::Fatal(f) => &f.message,
        }
    }
    pub fn get_file(&self) -> &str {
        match self {
            Diagnostic::Warning(w) => &w.file,
            Diagnostic::Info(i) => &i.file,
            Diagnostic::Error(e) => &e.file,
            Diagnostic::Fatal(f) => &f.file,
        }
    }
    pub fn get_location(&self) -> (usize, usize) {
        match self {
            Diagnostic::Warning(w) => (w.loc.0, w.loc.1),
            Diagnostic::Info(i) => (i.loc.0, i.loc.1),
            Diagnostic::Error(e) => (e.loc.0, e.loc.1),
            Diagnostic::Fatal(f) => (f.loc.0, f.loc.1),
        }
    }
    pub fn get_length(&self) -> usize{
        match self {
            Diagnostic::Warning(w) => w.len,
            Diagnostic::Info(i) => i.len,
            Diagnostic::Error(e) => e.len,
            Diagnostic::Fatal(f) => f.len,
        }
    }
    pub fn get_problem(&self) -> String{
        match &self{
            Diagnostic::Warning(w) => if w.code==0 {return "-".to_owned()},
            Diagnostic::Info(i) => if i.code==0 {return "-".to_owned()},
            Diagnostic::Error(e) => if e.code==0 {return "-".to_owned()},
            Diagnostic::Fatal(f) => if f.code==0 {return "-".to_owned()},
        }
        let binding=fs::read_to_string(match &self{
            Diagnostic::Warning(w) => &w.file,
            Diagnostic::Info(i) => &i.file,
            Diagnostic::Error(e) => &e.file,
            Diagnostic::Fatal(f) => &f.file,
        }).expect("Invalid Diagnostic! Failed to read to string!");
        let loc_idx=match &self{
            Diagnostic::Warning(w) => (w.loc.0, w.loc.1, w.len),
            Diagnostic::Info(i) => (i.loc.0, i.loc.1, i.len),
            Diagnostic::Error(e) => (e.loc.0, e.loc.1, e.len),
            Diagnostic::Fatal(f) => (f.loc.0, f.loc.1, f.len),
        };
        let spliced_iter=binding.lines().nth(loc_idx.0);
        return filestr[loc_idx.0][loc_idx.1..loc_idx.2].to_owned();
    }
}