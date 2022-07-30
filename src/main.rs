mod utils;
use std::{env, process::exit};
mod console;
mod core;

fn main() {
    const ARGS_MIN_SIZE: i8 = 2;
    let args: Vec<String> = env::args().collect();
    let args_length: i8 = args.len().try_into().unwrap();

    if args_length < ARGS_MIN_SIZE {
        println!("Please enter the command name");
        exit(1);
    }
    let name = &args[1];

    let cwd = utils::get_current_working_dir().unwrap();
    let config = utils::settings::get_settings(cwd);
    let scripts: Vec<utils::Scripts> =
        serde_json::from_reader(utils::settings::get_scripts_index(config.scripts_path))
            .expect("error while reading");

    for script in scripts {
        if name.to_owned() == script.command {
            let min_size: i8 = args_length - ARGS_MIN_SIZE;
            
            if min_size >= script.args.try_into().unwrap() {
                core::new_process(&script.path, &args[2..].to_vec());
            } else {
                println!("{:?}", script.command);
            }
        }
    }
}
