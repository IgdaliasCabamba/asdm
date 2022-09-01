use std::process::exit;
use crate::modules::data::Config;
use crate::modules::data::Data;
use crate::modules::console::Console;
use std::env;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Command {
    Run = 0,
    Install = 1,
    Setup = 2,
    Wrong = -1,
}
impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "run" => Ok(Command::Run),
            "install" => Ok(Command::Install),
            "setup" => Ok(Command::Setup),
            _ => Err(()),
        }
    }
}

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    return env::current_dir();
}
pub fn get_args_verify() -> Vec<String> {
    let mut args: Vec<String> = env::args().collect();
    args = args[1..].to_vec();
    return args;
}
pub fn cmd_to_enum(cmd: &str) -> Command {
    let res = Command::from_str(&cmd);
    if res.is_ok() {
        return res.ok().unwrap();
    } else {
        return Command::Wrong;
    }
}

pub mod settings {

    use super::{Config, Data};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::exit;
    use toml;

    pub fn get_settings(cwd: &PathBuf) -> Config {
        let filename: PathBuf = Path::new(&cwd)
            .join("src")
            .join("settings")
            .join("config.toml");

        let contents = match fs::read_to_string(&filename) {
            Ok(c) => c,
            Err(error) => {
                eprintln!(
                    "Out: Could not read file `{}`,
                    \nError:{}
                    \nplease be sure that file is in: settings/config.toml",
                    filename.into_os_string().into_string().unwrap(),
                    error
                );
                exit(1);
            }
        };
        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!(
                    "Unable to load data from `{}`",
                    &filename.into_os_string().into_string().unwrap()
                );
                exit(1);
            }
        };
        return data.config;
    }

    pub fn get_scripts_index(path: String) -> fs::File {
        let file = fs::File::open(path).expect("file not found");
        return file;
        //return fs::read_to_string(path).ok().unwrap();
    }
}

macro_rules! log_err {
    ($msg:expr, $term:expr) => {
        $term.print_error($msg)
    };
}

pub fn log_and_exit(msg:String, mut term:Console){
    log_err!(msg, term);
    exit(1)
}