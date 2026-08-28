use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::fmt::Display;
use std::{fs, io};
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use chrono::Local;
use itertools::Itertools;

mod tests;

/// Struct wrapper for the .properties format
#[derive(Clone, PartialEq, Eq, Default, Debug)]
pub struct Properties {
    values: HashMap<String, String>,
    path : Option<String>
}

impl Properties {

    /// Create a Properties struct from a string
    pub fn from_string(string: &str) -> Self {
        let mut map: HashMap<String, String> = Default::default();
        for line in string.lines() {

            if line.trim().starts_with("#") { continue };

            let mut keyval: Vec<&str> = line.split("=").collect::<Vec<&str>>();

            // if only one value
            if keyval.len() == 1 {
                keyval.push("");
            } else {
                if keyval.is_empty() {
                    continue;
                }
            }
            let _ = map
                .insert(keyval[0].to_string(), keyval[1].to_string());
        }
        Self { values: map, path: None }
    }

    /// Create a Properties from a text file
    pub fn from_file(path: &str) -> io::Result<Self> {
        let mut retval: Self;
        match fs::read_to_string(Path::new(path)) {
            Ok(string) =>  { retval = Properties::from_string(&string) } ,
            Err(e) => return Err(e)
        }
        retval.path = Some(path.to_string());
        Ok(retval)
    }

    /// Creates a new empty instance
    pub fn new() -> Self {
        Self {
            values: Default::default(),
            path: None,
        }
    }

    /// Add a key/value pair, replacing existing value if the key already exists
    pub fn add_set(&mut self, key: &str, value: &str) {
        self.values.insert(key.to_string(), value.to_string());
    }

    /// Removes a key pair value, returning the value if present
    pub fn remove(&mut self, key: String) -> Option<String> {
        self.values.remove(&key)
    }

    /// Get the value of a given key
    pub fn get(&self, key: &str) -> String {
        if let Some(value) = self.values.get(key) {
            value.clone()
        } else {
            panic!("Key \"{}\" not found!", key)
        }
    }

    /// Saves a file to the original location, assuming it was loaded from a file
    /// returns the path it was saved to.
    pub fn save(&self, comment : Option<&str>) -> io::Result<String> {
        match &self.path {
            None => {
                Err(Error::new(ErrorKind::NotFound, "No path specified for .properties file. Saving unsuccessful"))
            }
            Some(path) => {
                let now = Local::now();
                let mut file = File::open(path.clone())?;
                let mut write_string = format!("#{}\n{}",now.format("%a %b %d %H:%M:%S %Z %Y"),self.to_string());

                if let Some(text) = comment {
                    write_string = format!("#{}\n{}", text, write_string)
                }

                file.write_all(write_string.as_bytes())?;
                Ok(path.clone())
            }
        }
    }

    /// Saves a file to a given path
    /// returns the path it was saved to
    pub fn save_to(&self, path : &str, comment : Option<&str>) -> io::Result<String> {

        let now = Local::now();

        let file_path = Path::new(path);

        let mut file : File;

        if !Path::exists(file_path) {
            file = File::create(file_path)?;
        }
        else {
            file = File::open(file_path)?;
        }

        let mut write_string = format!("#{}\n{}",now.format("%a %b %d %H:%M:%S %Z %Y"),self.to_string());

        if let Some(text) = comment {
            write_string = format!("#{}\n{}", text, write_string)
        }

        file.write_all(write_string.as_bytes())?;

        Ok(path.to_string())

    }
}

impl Display for Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str: String = Default::default();
        for (key, value) in self.into_iter().sorted() {
            str = format!("{}{}={}\n", str, key, value)
        }
        write!(f, "{}", str.trim().to_string())
    }
}

impl<'a> IntoIterator for &'a Properties {
    type Item = (String, String);
    type IntoIter = IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.clone().into_iter()
    }
}
