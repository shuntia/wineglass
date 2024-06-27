use std::vec;
use std::collections::BTreeMap;

pub(super) struct ConfigSettings{
    content: String,
}

pub struct Config{
    children: BTreeMap<String, ConfigFolder>
}

pub(super) struct ConfigFolder{
    children: BTreeMap<String, FolderContent>
}

enum FolderContent{
    Folder(ConfigFolder),
    Setting(ConfigSettings),
}

impl Config{
    fn get_subfolder(&self) -> &vec<ConfigFolder>{
        let ret=vec![];
        for item in self.children{
            if(item.TypeID == ConfigFolder){
                vec.push(item.clone());
            }
        }
        return ret;
    }
    fn get_setting(&self) -> &vec<ConfigFolder>{
        let ret=vec![];
        for item in self.children{
            if(item.TypeID == ConfigFolder){
                vec.push(item.clone());
            }
        }
        return ret;
    }
    fn find_setting(&self, name: &str) -> vec<ConfigSettings>{
        
    }
    /*
    fn get_setting(&self, path: &str)->Result<ConfigSettings, &String>{
        let separated_path=path.split(":").collect();
        if(separated_path[0]!="Cfg"){
            return Err("Invalid root path for config: "+separated_path[0]);
        }
        let mut current_folder: ConfigFolder;
        for i in 0..self.children.len(){
            if(self.children[i].label == separated_path[1]){
                current_folder = self.children[i];
                break;
            }
            if(i==self.children.len()){
                return Err("Folder does not exist in specified path: Cfg:");
            }
        }
        for i in 2..separated_path.len()-1{
            for j in current_folder.children{
                if(j.label == separated_path[i]){
                    current_folder = j;
                    break;
                }
            }
            let mut err="";
            for k in 0..i{
                err += separated_path[k] + ":";
            }
            return Err("Folder does not exist in specified path: "+err);
        }
        for i in current_folder.children{
            if(i.label == separated_path[separated_path.len()-1]){
                return Ok(i);
            }
        }
        return Err(());
    }*/
    fn get_setting(&self, path: &str)->Result<ConfigSettings, &str>{
        separated_path=path.split(":").collect();
        if(separated_path[0]!="Cfg"){
            return Err("Invalid root path for config: "+separated_path[0]);
        }
        let mut current_folder: &ConfigFolder;
        for i in 1..separated_path.len()-1{
            current_folder=match current_folder.children[separated_path[i]]{
                FolderContent::Folder(f) => current_folder,
                FolderContent::Setting(s) => panic!("Setting with identical name as folder")
            };
        }
        return current_folder.children[separated_path[separated_path.len()-1]];
    }
    fn get_folder(&self, path:&str) -> vec<ConfigFolder>{
        separated_path=path.split(":").collect();
        current_folder=self.children
    }
}

impl ConfigFolder{
    fn get_subfolder(&self) -> &vec<ConfigFolder>{
        let ret=vec![];
        for item in self.children{
            if(item.TypeID == ConfigFolder){
                vec.push(item.clone());
            }
        }
        return ret;
    }
    fn get_setting(&self) -> &vec<ConfigFolder>{
        let ret=vec![];
        for item in self.children{
            if(item.TypeID == ConfigFolder){
                vec.push(item.clone());
            }
        }
        return ret;
    }
}