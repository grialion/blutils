use std::{
    any::Any, fmt::Display, fs::File, io::{Error, Read, Result}, path::Path, process::exit
};
use libc::getuid;

pub fn log<T: Display>(verbose: bool, message: T) {
    if verbose {
        println!("[log] {}", message)
    }
}

pub fn debug<T: Display>(debug: bool, message: T) {
    if debug {
        println!("[debug] {}", message)
    }
}

pub fn is_sudo() -> bool {
    unsafe {
        if getuid() != 0 {
            return false
        } else {
            return true;
        };
    };
}

// Stolen from https://stackoverflow.com/a/42773525
pub fn libc_wrap<T: Ord + Default>(num: T) -> Result<T> {
    if num < T::default() {
        return Err(Error::last_os_error());
    }
    Ok(num)
}

pub trait PathExtras {
    fn type_display(&self) -> Box<dyn Display>;
    fn ptype(&self) -> PathType;
}

impl PathExtras for Path {
    fn type_display(&self) -> Box<dyn Display> {
        if self.is_dir() {
            Box::new("directory")
        } else if self.is_symlink() {
            Box::new("symlink")
        } else {
            Box::new("file")
        }
    }
    fn ptype(&self) -> PathType {
        if self.is_dir() {
            PathType::Directory
        } else if self.is_symlink() {
            PathType::Symlink
        } else {
            PathType::File
        }
    }
}

pub enum PathType {
    File,
    Directory,
    Symlink
}

pub fn wrap<T: Any, M: Display>(result: Result<T>, prog: M) -> T {
    let val = match result {
        Ok(val) => val,
        Err(e) => {
            let mut error_code = 1;
            if let Some(os_error) = e.raw_os_error() {
                eprintln!("{}: Error: {}", prog, e.to_string());
                error_code = os_error;
            } else {
                eprintln!("{}: Error: {}", prog, e.to_string())
            };
            exit(error_code)
        }
    };
    return val;
}

pub fn prompt<T: Display>(question: T, d: bool) -> bool {
    let prompt_options = match d {
        true => "Y/n",
        false => "N/y",
    };
    println!("{} {}", prompt_options, question);
    let mut input = [0];
    let _ = std::io::stdin().read(&mut input);
    match input[0].to_ascii_lowercase() as char {
        'y' => true,
        'n' => false,
        _ => {return d}
    }
}
