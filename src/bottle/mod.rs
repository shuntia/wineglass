use std::fs;
use std::path::Path;
use crate::winecellar;
use crate::err;
pub fn start(path: &String) -> Result<i32, String> {
    assert_eq!(Path::new(path).exists(), true, "path does not exist!");
    let main_str:String=fs::read_to_string(path).expect("File unable to be read to string. Is the path valid?");
    winecellar::validate(main_str);
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