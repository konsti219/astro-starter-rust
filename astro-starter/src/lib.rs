use std::{env, path::Path};

mod config;
pub use crate::config::starter_config;

pub fn run() -> Result<(), String> {
    let work_dir = match env::current_dir() {
        Ok(dir) => dir,
        Err(_) => return Err("error getting current_dir".to_owned()),
    };

    println!("work_dir: {:?}", work_dir);

    let config_path = Path::new(&work_dir).join("starter_config.yml");
    let mut config = match starter_config::StarterConfig::new(&config_path) {
        Ok(c) => c,
        Err(err) => {
            let mut out = "Config error: ".to_owned();
            out.push_str(err);
            return Err(out);
        }
    };

    Ok(())
}
