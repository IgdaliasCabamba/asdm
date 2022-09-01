use crate::modules::console;
use crate::data::Installers;
use crate::core;
use std::process::exit;

pub fn install_app(name:&str, installers:Vec<Installers>, args:Vec<String>, args_length:i16, args_min_size:i16, mut term:console::Console){
    for installer in installers {
        if name.to_owned() == installer.app_name {
            let num_of_given_args: i16 = args_length - args_min_size;
            let num_of_args: i16 = installer.args.try_into().unwrap();

            if num_of_given_args == num_of_args {
                core::new_process(&installer.script, &args[2..].to_vec());
            } else if num_of_given_args < num_of_args {
                let mut msg = String::from(
                    "(Insufficient number of arguments)\t[this requires: &num]\t[you gave: &gave]");

                msg = msg.replace("&num", &installer.args.to_string());
                msg = msg.replace("&gave", &num_of_given_args.to_string());
                term.print_error(msg);
                exit(1);
            } else {
                let mut msg = String::from(
                    "(Too many arguments)\t[this requires: &num]\t[you gave: &gave]",
                );
                msg = msg.replace("&num", &installer.args.to_string());
                msg = msg.replace("&gave", &num_of_given_args.to_string());
                term.print_warning(msg);
                core::new_process(
                    &installer.script,
                    &args[2..=installer.args.try_into().ok().unwrap()].to_vec(),
                );
            }
        }
    }
}