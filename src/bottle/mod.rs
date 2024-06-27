use std::thread;
mod program;

pub mod parser{
    fn tokenize(s: &str) -> result<Vec<Token>, (i128, String)>{
        let mut tokens = Vec::new();
        for ln in s.split("\n").collect() {
            if(ln[0]=="#"){
                let bottle = ln.split(" ").collect();
                for tk in bottle{
                    
                }
            }
        }
    }
}