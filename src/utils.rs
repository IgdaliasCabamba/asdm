use std::path::PathBuf;
use std::env;
use serde_derive::Deserialize;

pub fn get_current_working_dir() -> std::io::Result<PathBuf> {
    return env::current_dir()
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

pub mod settings{  

    use std::fs;
    use std::process::exit;
    use toml;
    use std::path::{Path, PathBuf};
    use super::{Data, Config};

    pub fn get_settings(cwd:PathBuf) -> Config{
        
        let filename:PathBuf = Path::new(&cwd).join("src").join("settings").join("config.toml");

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
                eprintln!("Unable to load data from `{}`", &filename.into_os_string().into_string().unwrap());
                exit(1);
            }
        };
        return data.config;

    }

    pub fn get_scripts_index(path:String) -> fs::File{
        let file = fs::File::open(
            path        
        ).expect("file not found");
        return file;
        //return fs::read_to_string(path).ok().unwrap();
    }
}