#![forbid(unsafe_code)]
use clap::{CommandFactory, Parser, Subcommand};
use fsn::parse_file;
use std::{fs, path::Path, process};

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

fn main() {
    let default_conf_dir = dirs::home_dir().unwrap().join(".npi");
    let default_conf_dir = default_conf_dir.as_path();
    match default_conf_dir.try_exists() {
        Ok(b) => {
            if !b {
                if let Err(e) = fs::create_dir_all(&default_conf_dir) {
                    println!(
                        "Failed to create directory ({}): {e}",
                        &default_conf_dir.to_string_lossy()
                    )
                }
            }
        }
        Err(e) => println!("Can't read {}: {e}", &default_conf_dir.to_string_lossy()),
    }
    let args = Args::parse();
    if let Some(Commands::New { name, app_type }) = &args.command {
        println!("Creating project \"{name}\" of type \"{app_type}\"");
        let full_path = format!("{}/{}.fsn", &default_conf_dir.to_string_lossy(), &app_type);
        match parse_file(Path::new(&full_path)) {
            Ok(s) => {
                match fs::create_dir_all(format!("./{}", &name)) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Failed to create directory ({}): {e}", &name)
                    }
                }
                for dir in s.directories {
                    match fs::create_dir_all(format!("./{}/{}", &name, &dir)) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Failed to create directory ({dir}): {e}")
                        }
                    }
                }
                for file in s.files {
                    let custom_contents = &file.contents.replace(r"{{name}}", &name);
                    match fs::write(format!("./{}/{}", &name, file.name), custom_contents) {
                        Ok(_) => {}
                        Err(e) => println!("Failed to write to file ({}): {e}", file.name),
                    }
                }
            }
            Err(e) => println!("Error parsing the file {}: {e}", &full_path),
        }
    } else {
        _ = Args::command().print_help();
        process::exit(1);
    }
}
