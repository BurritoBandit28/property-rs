use std::collections::HashMap;
use std::collections::hash_map::IntoIter;
use std::fmt::Display;
use std::{fs, io};
use std::path::Path;

mod tests;

#[derive(Clone, PartialEq, Eq, Default)]
pub struct Properties {
    values: HashMap<String, String>,
}

impl Properties {

    pub fn from_string(string: &str) -> Self {
        let mut map: HashMap<String, String> = Default::default();
        for line in string.lines() {
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
                .insert(keyval[0].to_string(), keyval[1].to_string())
                .unwrap();
        }
        Self { values: map }
    }

    pub fn from_file(path: &str) -> io::Result<Self> {
        match fs::read_to_string(Path::new(path)) {
            Ok(string) => Ok(Properties::from_string(&string)),
            Err(e) => Err(e)
        }
    }

    pub fn new() -> Self {
        Self {
            values: Default::default(),
        }
    }

    pub fn add_set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
    }

    pub fn get(&self, key: String) -> String {
        if let Some(value) = self.values.get(&key) {
            value.clone()
        } else {
            panic!("Key \"{}\" not found!", key)
        }
    }
}

impl Display for Properties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str: String = Default::default();
        for (key, value) in self.into_iter() {
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
