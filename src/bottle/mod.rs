pub mod lexer;
use crate::config;
use log::*;
use std::path::{Path, PathBuf};

pub fn start(path: &String) -> Result<i32, String> {
    if !Path::new(path).exists() {
        panic!("File does not exist: {}", path);
    }
    info!("File found.");
    return Ok(0);
}
pub struct Bottle {
    pub state: i32,
    pub name: String,
    pub path: PathBuf,
    pub version: config::Version,
    pub description: String,
    pub lexer: lexer::Lexer,
}

impl Bottle {
    pub fn new<P: AsRef<Path>>(
        targetpath: P,
        name: Option<&str>,
        ver: Option<config::Version>,
        description: Option<&str>,
    ) -> Result<Bottle, String> {
        let path = targetpath.as_ref().to_path_buf();
        if !path.exists() {
            panic!("File does not exist: {}", path.display());
        }
        info!("Target found: {}", path.display());
        let file = match std::fs::File::open(&path) {
            Ok(f) => f,
            Err(e) => return Err(format!("Failed to open file: {}", e)),
        };
        let lexer = match lexer::Lexer::from_file(&file) {
            Ok(l) => l,
            Err(e) => return Err(format!("Failed to create lexer: {}", e)),
        };
        return Ok(Bottle {
            state: 0,
            name: match name {
                Some(n) => n.to_string(),
                None => path.file_name().unwrap().to_str().unwrap().to_string(),
            },
            path,
            version: match ver {
                Some(v) => v,
                None => config::Version::get_crate_ver(),
            },
            description: match description {
                Some(d) => d.to_string(),
                None => String::new(),
            },
            lexer,
        });
    }
    pub fn start(&mut self) {
        info!("Starting bottle: {}", self.name);
        for token in self.lexer.by_ref() {
            match token {
                Ok(t) => {
                    info!("Token: {:?}", t);
                }
                Err(e) => {
                    error!("Error: {}", e);
                }
            }
        }
    }
}
