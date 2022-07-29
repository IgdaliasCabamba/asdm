mod utils;

fn main(){
    let cwd = utils::get_current_working_dir().unwrap();
    let config = utils::settings::get_settings(cwd);
    println!("{}", config.scripts_path)
}