use std::thread;
pub mod parser {
    pub struct Parser<'a>{
        target_file: &mut String,
        target_site: &mut BindingSite,
        loc:i64
    }
    impl Parser{
        pub fn new() -> Parser{
        }
        pub fn attach(&mut self, site: &BindingSite){
            self.target_site=site;
        }
        pub fn attach(&mut self, target: &File, loc:i64){
            target
        }
    }
    pub struct BindingSite{
        pub target: &String,
        pub loc: i64,
    }
}