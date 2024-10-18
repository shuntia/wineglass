use std::env;
use std::fs::File;
use std::path::Path;
use std::path::MAIN_SEPARATOR_STR;
use std::vec;
mod bottle;
mod config;
mod err;
mod winecellar;
fn main() {
    env_logger::init();
    let args = env::args().collect::<Vec<String>>();
    println!("Starting Wineglass with arguments: {:?}", args);
    let target: Vec<String> = vec![
        env::current_dir().unwrap().to_str().unwrap().to_string(),
        "interpret\\main.wg".to_string(),
    ];
    println!("Target: {:?}", target.join(MAIN_SEPARATOR_STR));
    let sep: &str = MAIN_SEPARATOR_STR;
    let target_path_str = target.join(sep);
    let target_path = Path::new(&target_path_str);
    let com_args = config::Args::new();
    let file: File;
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
    println!("File found: {}", target_path.display());
    println!("Creating bottle...");
    let mut bottle = bottle::Bottle::new(target_path, None, None, None).unwrap();
    println!("Starting bottle!");
    bottle.start();
    println!("Bottle finished!");
}
