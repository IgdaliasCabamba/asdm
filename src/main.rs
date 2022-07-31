use std::process::exit;
use crate::console::Console;
mod console;
mod core;
mod utils;

fn main() {
    let mut console_api = Console::new();
    const ARGS_MIN_SIZE: i16 = 2;
    let args: Vec<String> = utils::get_args_verify(ARGS_MIN_SIZE);
    let args_length: i16 = args.len().try_into().unwrap();
    let name = &args[1];
    let cmd = utils::cmd_to_enum(&args[0]);
    let cwd = utils::get_current_working_dir().unwrap();
    let config = utils::settings::get_settings(cwd);
    let scripts: Vec<utils::Scripts> =
        serde_json::from_reader(utils::settings::get_scripts_index(config.scripts_path))
            .expect("error while reading");

    if cmd == utils::Command::Run {
        for script in scripts {
            if name.to_owned() == script.command {
                let num_of_given_args: i16 = args_length - ARGS_MIN_SIZE;
                let num_of_args: i16 = script.args.try_into().unwrap();

                if num_of_given_args == num_of_args {
                    core::new_process(&script.path, &args[2..].to_vec());
                } else if num_of_given_args < num_of_args {
                    let mut msg = String::from(
                        "(Insufficient number of arguments)\t[this requires: &num]\t[you gave: &gave]");

                    msg = msg.replace("&num", &script.args.to_string());
                    msg = msg.replace("&gave", &num_of_given_args.to_string());
                    console_api.print_error(msg);
                    exit(1);
                } else {
                    let mut msg = String::from(
                        "(Too many arguments)\t[this requires: &num]\t[you gave: &gave]"
                    );
                    msg = msg.replace("&num", &script.args.to_string());
                    msg = msg.replace("&gave", &num_of_given_args.to_string());
                    console_api.print_warning(msg);
                    core::new_process(&script.path, &args[2..=script.args.try_into().ok().unwrap()].to_vec());
                }
            }
        }
    } else {
        let msg = String::from("Command not found");
        console_api.print_error(msg);
    }
}
