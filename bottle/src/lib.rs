pub mod err;
use configmgr::config;
use log::*;
use std::path::{Path, PathBuf};
pub enum State {
    Standard(i32),
    Error(i32),
    Critical(i32),
    Paused(i32),
    Defective(i32),
    Fatal(i64),
    Shattered(i64),
}
pub struct Bottle {
    pub state: State,
    pub name: String,
    pub path: PathBuf,
    pub version: config::Version,
    pub description: String,
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
            return Err(format!("File does not exist: {}", path.display()));
        }
        info!("Target found: {}", path.display());
        return Ok(Bottle {
            state: State::Standard(0),
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
        });
    }
    pub fn start(&mut self) {
        println!("Starting bottle: {}", self.name);
        println!("Version: {}", self.version);
        println!("Description: {}", self.description);
        println!("Path: {}", self.path.display());
        println!("lexing...");
    }
}

//for errors
impl Bottle {
    pub fn error(&mut self, e: crate::err::Error) {
        self.state = State::Error(e.code);
        error!("Error in bottle {}: {}", self.name, e.message)
    }
    pub fn shatter(&mut self, report: crate::err::InternalReport) {
        self.state = State::Shattered(report.get_code());
        error!("Bottle shattered {}: {}", self.name, report.get_message());
    }
}
