use anyhow::Result;
use arboard::Clipboard;
use std::env;
use std::fmt::{Display, Error, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

pub fn copy(content: &str, register: Option<&str>, verbose: bool) -> Result<()> {
    match register {
        Some(filename) => {
            return copy_to_file(content, filename, verbose);
        }
        None => match Clipboard::new() {
            Ok(mut clipboard) => {
                clipboard.set_text(content.to_owned())?;
                if verbose {
                    println!("{}", &content);
                    println!("copied successfully to system clipboard");
                }
                return Ok(());
            }
            Err(_) => {
                return copy_to_file(content, "default", verbose);
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

pub fn get_dir() -> Result<PathBuf> {
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

fn get_register_paths() -> Result<Vec<PathBuf>> {
    return Ok(fs::read_dir(get_dir()?)?
        .map(|res| res.map(|e| e.path()))
        .filter_map(|x| x.ok())
        .filter(|x| x.is_file())
        .filter(|x| x.extension().is_some_and(|extension| extension == "txt"))
        .filter(|x| x.file_stem().is_some_and(|file_stem| !file_stem.is_empty()))
        .map(|path| path)
        .collect::<Vec<PathBuf>>());
}

fn copy_to_file(content: &str, filename: &str, verbose: bool) -> Result<()> {
    let dir = get_dir()?;
    fs::create_dir_all(&dir)?;
    let path = get_path(filename)?;
    fs::write(&path, &content)?;
    if verbose {
        println!("{}", &content);
        println!("copied successfully to register: {}", filename);
    }
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

pub fn list() -> Result<()> {
    let register_paths = get_register_paths()?;
    let mut register_names = register_paths
        .iter()
        .map(|x| String::from(x.file_stem().unwrap().to_str().unwrap()))
        .collect::<Vec<String>>();

    register_names.sort();

    println!("{}", StringVec(register_names));

    Ok(())
}

pub fn dump() -> Result<()> {
    let mut register_paths = get_register_paths()?;

    register_paths.sort();

    let register_contents = register_paths
        .iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .collect::<Vec<String>>();

    println!("{}", StringVec(register_contents));

    Ok(())
}

pub fn clear_all() -> Result<()> {
    let register_paths = get_register_paths()?;

    for path in register_paths {
        fs::remove_file(path)?;
    }

    Ok(())
}

struct StringVec(Vec<String>);

impl Display for StringVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut buffer = String::new();

        for item in &self.0 {
            buffer.push_str(item);
            buffer.push_str("\n");
        }

        write!(f, "{}", buffer.trim())
    }
}
