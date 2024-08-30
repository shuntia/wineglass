use std::any::TypeId;
use std::vec;
use std::collections::HashMap;
use std::env;
use log;

pub(super) enum Arg{
    File(String),
    Int(usize),
    Bool(bool)
}

pub(super) struct AllowedArgument{
    storeb:HashMap<String, bool>,
    storei:HashMap<String, u64>,
}
impl AllowedArgument {
    pub fn new() -> Self {
        Self {
            storeb: {
                let mut storeb = HashMap::new();
                storeb.insert("help".to_owned(), false);
                storeb.insert("debug".to_owned(), false);
                storeb.insert("version".to_owned(), false);
                storeb.insert("verbose".to_owned(), false);
                storeb.insert("trackerr".to_owned(), false);
                storeb.insert("continue".to_owned(), false);
                storeb.insert("log".to_owned(), false);
                // Add more arguments here
                storeb
            },
            storei: {
                let mut storei = HashMap::new();
                storei.insert("loglevel".to_owned(), 3);
                storei.insert("max-threads".to_owned(), 3);
                storei
            }
        }
    }
    pub fn find(&self, key: &str) -> Result<TypeId, String> {
        if self.storeb.contains_key(key){
            return Ok(TypeId::of::<HashMap<String, bool>>());
        }else if self.storei.contains_key(key){
            return Ok(TypeId::of::<HashMap<String, u64>>());
        }else{
            let mut e="Expected a valid map key, instead received ".to_owned();
            e.push_str(key);
            return Err(e);
        }
    }
}


enum Argtype{
    Bool,
    Int,
    String
}
pub struct Args{
    ///takes care of the arguments, including parsing.
    map: HashMap<String, Arg>
}
impl Args{
    pub fn new()->Self{
        let mut map: HashMap<String, Arg>=HashMap::new();
        let allowed_args = AllowedArgument::new();
        let v:Vec<String>=env::args().collect();
        for i in 1..v.len(){
            let current_arg = &v[i];
            if current_arg.starts_with("--") {
                let arg_content = current_arg[2..current_arg.len()].to_string();
                if allowed_args.find(&arg_content).expect("invalid args!")==TypeId::of::<HashMap<String, bool>>(){
                    map.insert(arg_content, Arg::Bool(true));
                }else if allowed_args.find(&arg_content).expect("invalid args!")==TypeId::of::<HashMap<String, u64>>(){
                    let tmp=Arg::Int(arg_content.parse::<usize>().expect("Invalid integer argument"));
                    map.insert(arg_content, tmp);
                }
            }
        }
        return Self{map:map};
    }
}
#[derive(Clone)]
pub(super) struct ConfigSettings{
    name: String,
    value: String,
}
#[derive(Clone)]
pub struct Config<'a>{
    name: String,
    children: HashMap<String, &'a ConfigFolder<'a>>
}
#[derive(Clone)]
pub(super) struct ConfigFolder<'a>{
    children: HashMap<String, &'a FolderContent<'a>>
}
#[derive(Clone)]
enum FolderContent<'a>{
    Folder(ConfigFolder<'a>),
    Setting(ConfigSettings),
}

impl<'a> Config<'a>{
    ///get all the subfolders in the root Cfg folder. Uses clone(), so watch out for time consumption.
    fn get_subfolder(&self) -> Result<HashMap<String, &ConfigFolder>, String>{
        return Ok(self.children.clone());
    }
    ///get all the settings that have the key given by DFS.
    fn find_setting(&self, name: &str) -> Vec<ConfigSettings> {
        let mut results = Vec::new();
        let mut stack : Vec<(String, &ConfigFolder<'a>)> = Vec::new();

        // Start with the root children
        for (key, content) in self.children.iter() {
            stack.push((key.clone(), &content.to_owned()));
        }

        // Perform DFS
        while let Some((_key, content)) = stack.pop() {
            for (child_key, child_content) in content.children.iter() {
                match child_content {
                    FolderContent::Folder(f)=>{
                        let x = f;
                        stack.push((child_key.clone(), &x))
                    },
                    FolderContent::Setting(s)=> {
                        if child_key == name {
                            results.push(s.clone());
                        }
                    }
                }
            }
        }

        return results
    }
    ///get a specific setting by its path.
    fn get_setting(&self, path: &str)->Result<&ConfigSettings, String>{
        let separated_path:Vec<_>=path.split(":").collect();
        if(separated_path[0]!="Cfg"){
            let mut e="Invalid root path for config: ".to_owned();
            e.push_str(separated_path[0]);
            return Err(e);
        }
        let mut current_folder: &ConfigFolder=self.children[path];
        for i in 1..separated_path.len()-1{
            current_folder=match &current_folder.children[separated_path[i]]{
                FolderContent::Folder(f) => current_folder,
                FolderContent::Setting(s) => panic!("Setting with identical name as folder")
            };
        }
        match &current_folder.children[separated_path[separated_path.len()-1]]{
            FolderContent::Folder(f) => {
                let mut e="Folder with identical name: ".to_owned();
                e.push_str(separated_path[separated_path.len()-1]);
                return Err(e);
            },
            FolderContent::Setting(s) => Ok(&s)
        }
    }
    ///get a specific folder by its path.
    fn get_folder(&self, path:&str) -> Result<ConfigFolder, String>{
        let separated_path:Vec<_>=path.split(":").collect();
        let mut current_folder: ConfigFolder;
        if separated_path[1]!="Cfg"{
            let mut e="Invalid root path for config: ".to_owned();
            e.push_str(separated_path[1]);
            return Err(e);
        }
        if self.children.contains_key(separated_path[1]){
            current_folder = self.children[separated_path[1]].clone();
        }else{
            let mut e="Folder does not exist in specified path: Cfg:".to_owned();
            e.push_str(separated_path[1]);
            return Err(e);
        }
        for i in 1..separated_path.len(){
            if current_folder.children.contains_key(separated_path[i]){
                current_folder = match &current_folder.children[separated_path[1]]{
                    FolderContent::Folder(f) => f.clone(),
                    FolderContent::Setting(s) => {let mut e="Setting with identical name: ".to_owned(); e.push_str(separated_path[1]);return Err(e)}
                };
            }
        }
        return Ok(current_folder.clone());
    }
}

impl<'a> ConfigFolder<'a>{
    fn get_subfolder(&self) -> Result<Vec<&ConfigFolder>, String>{
        let mut ret: Vec<&ConfigFolder>=vec![];
        for item in &self.children{
            match item.1{
                FolderContent::Folder(f) => ret.push(f),
                FolderContent::Setting(s) => {
                    let mut e="Setting with identical name: ".to_owned();
                    e.push_str(s.name.as_str());
                    return Err(e);
                }
            }
        }
        return Ok(ret);
    }
    fn get_subsetting(&self) -> Vec<ConfigSettings>{
        let mut ret=vec![];
        for item in &self.children{
            let c=item.1;
            match c{
                FolderContent::Folder(_f)=>(),
                FolderContent::Setting(s)=>ret.push(s.to_owned())
            }
        }
        return ret;
    }
}
