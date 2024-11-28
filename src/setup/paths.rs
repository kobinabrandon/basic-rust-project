use std::{fs, io::ErrorKind, path::PathBuf};
use log::{info, error};


pub struct Paths {
    pub models: PathBuf,
    pub data: PathBuf,
}


impl Paths {
    pub fn setup() -> Self {
        let current_dir = std::env::current_dir().expect("Failed to provide current directory");
       
        Self {
            models: current_dir.join("models"),
            data: current_dir.join("data")
        }
    }
}


pub fn make_directories() {
    info!("Creating directories");

    let paths = Paths::setup(); 
    let mut paths_to_make = Vec::new();
    
    paths_to_make.push(paths.data);
    paths_to_make.push(paths.models);

    for path in paths_to_make { 
        match fs::create_dir(path.clone()) {
            Ok(_) => info!("Created {} directory", path.display()),
            Err(e) => {
                if e.kind() == ErrorKind::AlreadyExists {
                    error!("Directory already exists: {}", path.display());
                }
                else {
                    error!("Could not create directory: {}", e);
                }
            }
        }                
    } 

}

