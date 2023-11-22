use crate::env::get_base_path;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub path: String,
    pub is_dir: bool,
}

impl Entry {
    pub fn new(dir_entry: &std::fs::DirEntry) -> Self {
        let path = (*dir_entry)
            .file_name()
            .into_string()
            .unwrap()
            .replace(get_base_path().as_str(), "");

        let is_dir = (*dir_entry).path().is_dir();

        Self { path, is_dir }
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            self.path,
            if self.is_dir { "DIR" } else { "FILE" }
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Files {
    pub path: String,
    pub entries: Vec<Entry>,
}

impl Files {
    pub fn from_path(path: String) -> Result<Self, std::io::Error> {
        let entries = Self::read_path(&path)?;

        Ok(Self { path, entries })
    }

    pub fn read_path(path: &String) -> Result<Vec<Entry>, std::io::Error> {
        let result = std::fs::read_dir((*path).as_str())?
            .filter_map(|x| x.ok())
            .map(|x| Entry::new(&x))
            .collect::<Vec<_>>();

        Ok(result)
    }
}

/**
 * use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("/mnt/storage/distros/lfs/disk.qcow2").unwrap();
    let mut buffer = Vec::<u8>::new();
    let mut chunk = [0; 4096]; // Read 4096 bytes at a time

    loop {
        match file.read(&mut chunk) {
            Ok(0) => break, // Reached the end of the file
            Ok(n) => buffer.extend_from_slice(&chunk[..n]),
            Err(err) => panic!("Failed reading buffer: {}", err),
        }
    }

    println!("File read successfully!");
}

 */