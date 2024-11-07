use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::path::MAIN_SEPARATOR_STR;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    let args = env::args().collect::<Vec<String>>();
    let target: Vec<String> = vec![
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        "interpret\\main.wg".to_string(),
    ];
    println!("Target: {:?}", target.join(MAIN_SEPARATOR_STR));
    let sep: &str = MAIN_SEPARATOR_STR;
    let target_path_str = target.join(sep);
    let target_path = Path::new(&target_path_str);
    let com_args = configmgr::config::Args::new();
    let mut file: File;
    match target_path.exists() {
        true => {
            file = match File::open(target_path) {
                Err(why) => panic!("FATAL! Couldn't open {}: {}", target_path.display(), why),
                Ok(file) => file,
            };
        }
        false => panic!("FATAL! Target file not found: {}", target_path.display()),
        _ => panic!(
            "FATAL! Unknown error! failed to check if target path exists!: {}",
            target_path.display()
        ),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("FATAL! Couldn't read {}: {}", target_path.display(), why),
        Ok(_) => println!("Contents: {}", contents),
    };
    let parsed = parser::parse(&contents);
    println!("Parsed: {:#?}", parsed.unwrap());
}
