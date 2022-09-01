mod core;
mod utils;
mod modules;
use modules::console::Console;
use modules::{data, setup, run, install};
use utils::log_and_exit;

fn main() {
    let mut console_api = Console::new();
    const ARGS_MIN_SIZE: i16 = 2;
    const ARGS_MIN_SIZE_TO_RUN: i16 = 1;
    let args: Vec<String> = utils::get_args_verify();
    let args_length: i16 = args.len().try_into().unwrap();
    let cwd = utils::get_current_working_dir().unwrap();
    let config = utils::settings::get_settings(&cwd);

    if args_length < ARGS_MIN_SIZE_TO_RUN{
        log_and_exit(String::from("Enter the Command"), console_api)
    }
    else if args.len() < ARGS_MIN_SIZE.try_into().unwrap() {
        log_and_exit(String::from("Enter the Script"), console_api)
    }
    else {
        let cmd = utils::cmd_to_enum(&args[0]);
        let name = &args[1];
        let run_scripts: Vec<data::Scripts> =
            serde_json::from_reader(utils::settings::get_scripts_index(config.run_scripts_path))
                .expect("error while reading");
        let install_scripts: Vec<data::Installers> =
        serde_json::from_reader(utils::settings::get_scripts_index(config.install_scripts_path))
            .expect("error while reading"); 

        if cmd == utils::Command::Run {
            run::run_script(&name, run_scripts, args.clone().to_vec(), args_length, ARGS_MIN_SIZE, console_api);

        } else if cmd == utils::Command::Install {
            install::install_app(&name, install_scripts, args.clone().to_vec(), args_length, ARGS_MIN_SIZE, console_api);
        }
        else if cmd == utils::Command::Setup {
            setup::setup_app(&cwd);
        }else {
            let msg = String::from("Command not found");
            console_api.print_error(msg);
        }
    }
}
