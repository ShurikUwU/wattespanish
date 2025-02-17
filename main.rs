use std::fs;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "wattespanish", about = "WatteSpanish file manager.")]
enum Command {
    /// List contents of a directory
    List {
        #[structopt(parse(from_os_str))]
        path: Option<std::path::PathBuf>,
    },
    /// Create a new directory
    Mkdir {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
    /// Remove a file or directory
    Remove {
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
    },
}

fn list_directory(path: Option<&Path>) {
    let path = path.unwrap_or_else(|| Path::new("."));
    if path.is_dir() {
        println!("Contents of directory '{}':", path.display());
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.path().display());
                }
            }
        }
    } else {
        eprintln!("'{}' is not a directory.", path.display());
    }
}

fn create_directory(path: &Path) {
    if let Err(e) = fs::create_dir(path) {
        eprintln!("Failed to create directory '{}': {}", path.display(), e);
    } else {
        println!("Directory '{}' created successfully.", path.display());
    }
}

fn remove_path(path: &Path) {
    if path.is_dir() {
        if let Err(e) = fs::remove_dir(path) {
            eprintln!("Failed to remove directory '{}': {}", path.display(), e);
        } else {
            println!("Directory '{}' removed successfully.", path.display());
        }
    } else if path.is_file() {
        if let Err(e) = fs::remove_file(path) {
            eprintln!("Failed to remove file '{}': {}", path.display(), e);
        } else {
            println!("File '{}' removed successfully.", path.display());
        }
    } else {
        eprintln!("'{}' does not exist.", path.display());
    }
}

fn main() {
    let cmd = Command::from_args();

    match cmd {
        Command::List { path } => list_directory(path.as_deref()),
        Command::Mkdir { path } => create_directory(&path),
        Command::Remove { path } => remove_path(&path),
    }
}
