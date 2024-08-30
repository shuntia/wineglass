use std::fs;
use std::path::Path;
use crate::winecellar;
use crate::err;
use log::{info, warn, error, trace};
pub fn start(path: &String) -> Result<i32, String> {
    if !Path::new(path).exists() {
        panic!("File does not exist: {}", path);
    }
    log::info!("File found.");
    winecellar::validate(&mut match fs::File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("Error opening file for unknown reason! {:?}", why),
    });
    return Ok(0);
}





pub struct Lexer{
    pos:i128,
    input:Vec<char>,
}
impl Lexer{
    fn new(input:String)->Self{
        Self{
            pos:0,
            input:input.chars().collect(),
        }
    }
    fn next_token(&mut self) -> Result <i128, String> {
        //outputs next token in the input provided at Lexer::new()
        return Ok(0);
    }
}