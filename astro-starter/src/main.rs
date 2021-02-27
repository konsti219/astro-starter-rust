use std::{env, path::Path, process};

mod config;
pub use crate::config::starter_config;

fn main() {
    println!("Astro Starter 0.1.0");

    let work_dir = env::current_dir().expect("Could not get work_dir");

    let mut config = starter_config::StarterConfig::new(&work_dir);
}
