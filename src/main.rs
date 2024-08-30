use std::fs::File;
use std::path::Path;
use std::path::MAIN_SEPARATOR_STR;
use std::vec;
use std::env;
use log::*;
use env_logger;
mod winecellar;
mod bottle;
mod config;
pub mod err;
fn main() {
    env_logger::init();
    println!("Starting Wineglass with arguments: {:?}", env::args().collect::<Vec<String>>());
    let target:Vec<String>=vec![env!("CARGO_MANIFEST_DIR").to_string(),"interpret\\main.wg".to_string()];
    let sep:&str = &MAIN_SEPARATOR_STR;
    let target_path_str = target.join(sep);
    let target_path = Path::new(&target_path_str);
    let com_args = config::Args::new();
    let mut file:File;
    match target_path.exists(){
        true => {
            file = match File::open(&target_path) {
                Err(why) => panic!("FATAL! Couldn't open {}: {}", target_path.display(), why),
                Ok(file) => file
            };
        }
        false => panic!("FATAL! Target file not found: {}", target_path.display()),
        _ => panic!("FATAL! Unknown error! failed to check if target path exists!: {}", target_path.display())
    };
    let diag = match winecellar::validate(&mut file) {
        Err(why) => panic!("Couldn't validate {}: Due to {}", target_path.display(), why),
        Ok(vec) => vec,
    };
    if diag.len()==0 {
        println!("All Clear");
    }else{
        for i in diag {
            match i {
                err::Diagnostic::Error(e) => {
                    println!("Error: {} at {:?}", e.message, e.loc);
                },
                err::Diagnostic::Warning(w) => {
                    println!("Warning: {} at {:?}", w.message, w.loc);
                },
                err::Diagnostic::Info(info) => {
                    println!("Info: {} at {:?}", info.message, info.loc);
                },
                err::Diagnostic::Fatal(f) => {
                    println!("Fatal: {} at {:?}", f.message, f.loc);
                },
            }
        }
    }
}