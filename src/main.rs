use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::vec;
mod winecellar;

fn main() {
    let path = Path::new("C:/Users/shunt/playground/wineglass/interpret/main.wg");
    let display = path.display();
    
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    
    let diag = match winecellar::validate(&mut file) {
        Err(why) => panic!("couldn't validate {}: Due to {}", display, why),
        Ok(vec) => vec,
    };
    if diag.len()==0 {
        println!("All Clear")
    }
    for i in diag {
        match i {
            winecellar::Diagnostic::Error(e) => {
                println!("Error: {} at {:?}", e.message, e.loc);
            },
            winecellar::Diagnostic::Warning(w) => {
                println!("Warning: {} at {:?}", w.message, w.loc);
            },
            winecellar::Diagnostic::Info(info) => {
                println!("Info: {} at {:?}", info.message, info.loc);
            },
            winecellar::Diagnostic::Fatal(f) => {
                println!("Fatal: {} at {:?}", f.message, f.loc);
            },
        }
    }
}
