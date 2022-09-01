use std::path::PathBuf;

pub fn setup_app(cwd:&PathBuf){
    println!("Ok from: {}", cwd.display())
}