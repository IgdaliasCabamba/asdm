use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Scripts {
    pub command: String,
    pub path: String,
    pub args: i16,
}

#[derive(Debug, Deserialize)]
pub struct Installers {
    pub app_name: String,
    pub script: String,
    pub args: i16,
}


#[derive(Debug, Deserialize)]
pub struct Data {
    pub config: Config,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub run_scripts_path: String,
    pub install_scripts_path: String,
}
