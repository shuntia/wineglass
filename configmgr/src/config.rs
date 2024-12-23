use std::any::TypeId;
use std::collections::HashMap;
use std::env;
use std::sync::OnceLock;

pub(super) enum Arg {
    File(String),
    Int(usize),
    Bool(bool),
}

pub(super) struct AllowedArgument {
    storeb: HashMap<String, bool>,
    storei: HashMap<String, u64>,
}
impl AllowedArgument {
    pub fn new() -> Self {
        Self {
            storeb: {
                let mut storeb = HashMap::new();
                storeb.insert("help".to_owned(), false);
                storeb.insert("verbose".to_owned(), false);
                storeb.insert("info".to_owned(), false);
                storeb.insert("debug".to_owned(), false);
                storeb.insert("version".to_owned(), false);
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
            },
        }
    }
    pub fn find(&self, key: &str) -> Result<TypeId, String> {
        if self.storeb.contains_key(key) {
            Ok(TypeId::of::<HashMap<String, bool>>())
        } else if self.storei.contains_key(key) {
            return Ok(TypeId::of::<HashMap<String, u64>>());
        } else {
            let mut e = "Expected a valid map key, instead received ".to_owned();
            e.push_str(key);
            return Err(e);
        }
    }
}

enum Argtype {
    Bool,
    Int,
    String,
}

pub struct Args {
    ///takes care of the arguments, including parsing.
    map: HashMap<String, Arg>,
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Self {
        let mut map: HashMap<String, Arg> = HashMap::new();
        let allowed_args = AllowedArgument::new();
        let v: Vec<String> = env::args().collect();
        for i in 1..v.len() {
            let current_arg = &v[i];
            if current_arg.starts_with("--") {
                let arg_content = current_arg[2..current_arg.len()].to_string();
                if allowed_args.find(&arg_content).expect("invalid args!")
                    == TypeId::of::<HashMap<String, bool>>()
                {
                    map.insert(arg_content, Arg::Bool(true));
                } else if allowed_args.find(&arg_content).expect("invalid args!")
                    == TypeId::of::<HashMap<String, u64>>()
                {
                    let tmp = Arg::Int(
                        arg_content
                            .parse::<usize>()
                            .expect("Invalid integer argument"),
                    );
                    map.insert(arg_content, tmp);
                }
            }
        }
        Args { map }
    }
}

impl Args {
    pub fn get(&self, key: &str) -> Option<&Arg> {
        self.map.get(key)
    }

    pub fn apply(&self) {
        if match self.get("debug") {
            Some(Arg::Bool(b)) => *b,
            _ => false,
        } {
            log::set_max_level(log::LevelFilter::Debug);
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(Config::new)
}

pub(super) struct ConfigSettings {
    name: String,
    value: String,
}

pub struct Config {
    name: String,
    children: HashMap<String, &'static ConfigFolder>,
}

pub(super) struct ConfigFolder {
    children: HashMap<String, &'static FolderContent>,
}

enum FolderContent {
    Folder(ConfigFolder),
    Setting(ConfigSettings),
}
impl Config {
    /// Loads the config file from the disk. This is not recommended to check config.
    /// use get_config() instead.
    pub fn load() -> Config {
        let mut config = Config::new();
        config.name = "config".to_string();

        config
    }
    fn new() -> Self {
        Config {
            name: String::new(),
            children: HashMap::new(),
        }
    }
    ///
    pub fn init() -> &'static Config {
        CONFIG.get_or_init(Config::load)
    }
    /// Get all the subfolders in the root Cfg folder. Uses clone(), so watch out for time consumption.
    fn get_subfolder(&self) -> Result<HashMap<String, &ConfigFolder>, String> {
        Ok(self.children.clone())
    }

    /// Get all the settings that have the key given by DFS.
    fn find_setting(&self, name: &str) -> Vec<ConfigSettings> {
        let results = Vec::new();
        let stack: Vec<(String, &ConfigFolder)> = Vec::new();
        results
    }
}

pub struct Version {
    major: i32,
    minor: i32,
    patch: i32,
}

impl Version {
    pub fn new(major: i32, minor: i32, patch: i32) -> Version {
        Version {
            major,
            minor,
            patch,
        }
    }
    pub fn get_crate_ver() -> Version {
        Version {
            major: env!("CARGO_PKG_VERSION_MAJOR").parse::<i32>().unwrap(),
            minor: env!("CARGO_PKG_VERSION_MINOR").parse::<i32>().unwrap(),
            patch: env!("CARGO_PKG_VERSION_PATCH").parse::<i32>().unwrap(),
        }
    }
    pub fn major(&self) -> i32 {
        self.major
    }
    pub fn minor(&self) -> i32 {
        self.minor
    }
    pub fn patch(&self) -> i32 {
        self.patch
    }
}
impl From<&str> for Version {
    fn from(s: &str) -> Version {
        let v: Vec<&str> = s.split('.').collect();
        Version {
            major: v[0].parse::<i32>().unwrap(),
            minor: v[1].parse::<i32>().unwrap(),
            patch: v[2].parse::<i32>().unwrap(),
        }
    }
}
impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
