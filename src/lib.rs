use anyhow::Result;
use arboard::Clipboard;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy(content: &str, register: Option<&str>) -> Result<()> {
    match register {
        Some(filename) => {
            return copy_to_file(content, filename);
        }
        None => match Clipboard::new() {
            Ok(mut clipboard) => {
                clipboard.set_text(content.to_owned())?;
                println!("{}", &content);
                println!("copied successfully to system clipboard");
                return Ok(());
            }
            Err(_) => {
                return copy_to_file(content, "default");
            }
        },
    };
}

pub fn paste(register: Option<&str>) -> Result<()> {
    match register {
        Some(filename) => {
            let path = get_path(filename)?;
            return paste_from_file(path);
        }
        None => match Clipboard::new() {
            Ok(mut clipboard) => {
                let text = clipboard.get_text()?;
                println!("{}", text);
                return Ok(());
            }
            Err(_) => {
                let path = get_path("default")?;
                return paste_from_file(path);
            }
        },
    };
}

fn get_path(filename: &str) -> Result<PathBuf> {
    let mut path = get_dir()?;
    path.push(Path::new(filename).file_name().unwrap());
    path.set_extension("txt");
    return Ok(path);
}

fn get_dir() -> Result<PathBuf> {
    let mut path;
    match env::var("CB_DIR") {
        Ok(cb_dir_str) => {
            path = PathBuf::from(cb_dir_str);
        }
        Err(_) => {
            path = env::temp_dir();
            path.push("crepe-bordeaux");
        }
    };
    return Ok(path);
}

fn copy_to_file(content: &str, filename: &str) -> Result<()> {
    let dir = get_dir()?;
    fs::create_dir_all(&dir)?;
    let path = get_path(filename)?;
    fs::write(&path, &content)?;
    println!("{}", &content);
    println!("copied successfully to register: {}", filename);
    return Ok(());
}

fn paste_from_file(filepath: PathBuf) -> Result<()> {
    match filepath.is_file() {
        true => {
            let x = fs::read_to_string(filepath)?;
            println!("{}", x);
            return Ok(());
        }
        false => {
            print!("");
            return Ok(());
        }
    };
}
