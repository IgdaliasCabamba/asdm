use serde_derive::Deserialize;
use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Command {
    Run = 0,
    Wrong = -1,
}
impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "run" => Ok(Command::Run),
            _ => Err(()),
        }
    }
}

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    return env::current_dir();
}
pub fn get_args_verify(min_size: i16) -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < min_size.try_into().unwrap() {
        println!("Please enter the command name");
        exit(1);
    }

    return args[1..].to_vec();
}
pub fn cmd_to_enum(cmd: &str) -> Command {
    let res = Command::from_str(&cmd);
    if res.is_ok() {
        return res.ok().unwrap();
    } else {
        return Command::Wrong;
    }
}

#[derive(Debug, Deserialize)]
pub struct Scripts {
    pub command: String,
    pub path: String,
    pub args: i16,
}

#[derive(Debug, Deserialize)]
struct Data {
    config: Config,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub scripts_path: String,
}

pub mod settings {

    use super::{Config, Data};
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::exit;
    use toml;

    pub fn get_settings(cwd: PathBuf) -> Config {
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
