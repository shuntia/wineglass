use crate::err;
use std::io::Read;
pub fn validate(file: &mut std::fs::File) -> Result<Vec<err::Diagnostic>, String> {
    let valid = file;
    let mut contents = String::new();
    valid
        .read_to_string(&mut contents)
        .map_err(|_| "Error reading file".to_string())?;
    return validate_str(&contents);
}
pub fn validate_str(file: &str) -> Result<Vec<err::Diagnostic>, String> {
    return Ok(vec![]);
}
