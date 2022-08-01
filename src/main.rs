use utils::Scripts;
use crate::console::Console;
use std::process::exit;
mod console;
mod core;
mod utils;

macro_rules! log_err {
    ($msg:expr, $term:expr) => {
        $term.print_error($msg)
    };
}

fn log_and_exit(msg:String, mut term:Console){
    log_err!(msg, term);
    exit(1)
}

fn run_script(name:&str, scripts:Vec<Scripts>, args:Vec<String>, args_length:i16, args_min_size:i16, mut term:Console){
    for script in scripts {
        if name.to_owned() == script.command {
            let num_of_given_args: i16 = args_length - args_min_size;
            let num_of_args: i16 = script.args.try_into().unwrap();

            if num_of_given_args == num_of_args {
                core::new_process(&script.path, &args[2..].to_vec());
            } else if num_of_given_args < num_of_args {
                let mut msg = String::from(
                    "(Insufficient number of arguments)\t[this requires: &num]\t[you gave: &gave]");

                msg = msg.replace("&num", &script.args.to_string());
                msg = msg.replace("&gave", &num_of_given_args.to_string());
                term.print_error(msg);
                exit(1);
            } else {
                let mut msg = String::from(
                    "(Too many arguments)\t[this requires: &num]\t[you gave: &gave]",
                );
                msg = msg.replace("&num", &script.args.to_string());
                msg = msg.replace("&gave", &num_of_given_args.to_string());
                term.print_warning(msg);
                core::new_process(
                    &script.path,
                    &args[2..=script.args.try_into().ok().unwrap()].to_vec(),
                );
            }
        }
    }
}

fn main() {
    let mut console_api = Console::new();
    const ARGS_MIN_SIZE: i16 = 2;
    const ARGS_MIN_SIZE_TO_RUN: i16 = 1;
    let args: Vec<String> = utils::get_args_verify();
    let args_length: i16 = args.len().try_into().unwrap();
    let cwd = utils::get_current_working_dir().unwrap();
    let config = utils::settings::get_settings(cwd);

    if args_length < ARGS_MIN_SIZE_TO_RUN{
        log_and_exit(String::from("Enter the Command"), console_api)
    }
    else if args.len() < ARGS_MIN_SIZE.try_into().unwrap() {
        log_and_exit(String::from("Enter the Script"), console_api)
    }
    else {
        let cmd = utils::cmd_to_enum(&args[0]);
        let name = &args[1];
        let run_scripts: Vec<utils::Scripts> =
            serde_json::from_reader(utils::settings::get_scripts_index(config.run_scripts_path))
                .expect("error while reading");
        let install_scripts: Vec<utils::Scripts> =
        serde_json::from_reader(utils::settings::get_scripts_index(config.install_scripts_path))
            .expect("error while reading"); 

        if cmd == utils::Command::Run {
            run_script(&name, run_scripts, args.clone().to_vec(), args_length, ARGS_MIN_SIZE, console_api);

        } else if cmd == utils::Command::Install {
            run_script(&name, install_scripts, args.clone().to_vec(), args_length, ARGS_MIN_SIZE, console_api);

        }else {
            let msg = String::from("Command not found");
            console_api.print_error(msg);
        }
    }
}
