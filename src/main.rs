use crate::config::Config;
use crate::file_manager::file_walk;
use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::process;

mod config;
mod file_manager;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem with arguments {err}");
        process::exit(1);
    });

    let path_to_files = Path::new(&config.path);
    let file_option = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open("files.txt");

    match file_option {
        Err(e) => {
            println!("Error openning file {e}");
        }
        Ok(file) => {
            if let Err(e) = file_walk(
                path_to_files,
                &file,
                &config.exclude_file,
                config.no_extension,
            ) {
                println!("ERROR {e}");
                process::exit(1);
            }
        }
    }
}
