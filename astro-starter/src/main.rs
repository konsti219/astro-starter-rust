use std::{
    io::{stdin, stdout, Write},
    process,
};

use astro_starter;

fn main() {
    println!("Astro Starter 0.1.0");

    if let Err(e) = astro_starter::run() {
        println!("Application error: {}", e);

        pause();
        process::exit(1);
    }
}

fn pause() {
    print!("Press Enter to continue... ");
    let _ = stdout().flush();
    stdin().read_line(&mut String::new()).unwrap();
}
