use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Warning {
    pub file: String,
    pub message: String,
    pub loc: (i64, i64),
}

pub struct Info {
    pub file: String,
    pub message: String,
    pub loc: (i64, i64),
}

pub struct Error {
    pub file: String,
    pub message: String,
    pub loc: (i64, i64),
}

pub struct Fatal {
    pub file: String,
    pub message: String,
    pub loc: (i64, i64),
}

pub enum Diagnostic {
    Warning(Warning),
    Info(Info),
    Error(Error),
    Fatal(Fatal),
}

pub fn validate(file: &mut std::fs::File) -> Result<Vec<Diagnostic>, String> {
    // Dummy implementation for validation
    return Ok(vec![]);
}
