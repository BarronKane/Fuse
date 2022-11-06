

use std::env;
use std::io;
use std::path::PathBuf;

pub fn get_cwd() -> io::Result<PathBuf> {
    let mut pwd = env::current_exe()?;
    pwd.pop();
    Ok(pwd)
}

pub fn get_main() -> io::Result<PathBuf> {
    let exe = env::current_exe()?;
    Ok(exe)
}

pub fn file_to_bytes(path: std::path::PathBuf) -> Vec<u8> {
    let bytes = std::fs::read(path).unwrap();
    return bytes;
}

pub fn file_to_string(path: std::path::PathBuf) -> String {
    std::fs::read_to_string(path).unwrap()
}
