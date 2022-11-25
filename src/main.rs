#![forbid(unsafe_code)]
use clap::{CommandFactory, Parser, Subcommand};
use fsn::parse_file;
use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

#[derive(Parser)]
#[clap(name = "New Project Initializer", author, version, about)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,

    /// Display every step
    #[clap(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Create new directory with a name
    New {
        /// Directory name
        #[clap(value_parser, name = "name")]
        name: String,
        /// Project type to be inited
        #[clap(value_parser, name = "type")]
        app_type: String,
    },
}

fn get_config_dir() -> Result<PathBuf, String> {
    let default_conf_dir = dirs::config_dir().unwrap().join("npi");
    if !default_conf_dir.exists() {
        if let Err(e) = fs::create_dir_all(&default_conf_dir) {
            return Err(format!(
                "Failed to create config directory ({}): {e}",
                default_conf_dir.to_string_lossy()
            ));
        }
    }
    Ok(default_conf_dir)
}

fn main() {
    let conf_dir = match get_config_dir() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            process::exit(1);
        }
    };

    let args = Args::parse();
    if let Some(Commands::New { name, app_type }) = &args.command {
        println!("Creating project \"{name}\" of type \"{app_type}\"");
        let full_path = conf_dir.join(format!("{app_type}.fsn"));
        match parse_file(&full_path) {
            Ok(s) => {
                if let Err(e) = fs::create_dir_all(&name) {
                    eprintln!("Failed to create directory ({name}): {e}")
                }
                for dir in s.directories {
                    if let Err(e) = fs::create_dir_all(Path::new(name).join(&dir)) {
                        eprintln!("Failed to create directory ({dir}): {e}")
                    }
                }
                for file in s.files {
                    let custom_contents = file.contents.replace(r"{{name}}", name);
                    if let Err(e) = fs::write(Path::new(name).join(&file.name), custom_contents) {
                        eprintln!("Failed to write to file ({}): {e}", file.name);
                    }
                }
            }
            Err(e) => println!(
                "Error parsing the file {}: {e}",
                full_path.to_string_lossy()
            ),
        }
    } else {
        _ = Args::command().print_help();
        process::exit(1);
    }
}
