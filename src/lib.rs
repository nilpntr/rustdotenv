//! A tool to load env files into the environment
//! # (Pseudo-)Example:
//! ```ignore
//! ```

use std::collections::HashMap;
use std::io;
use std::path::Path;
use envfile::EnvFile;

/// Check if the provided env name equals true like DEV=true, if so it runs the load function with the provided filenames
pub fn load_if_set(env_name: &str, _filenames: Option<Vec<&str>>) {
    let dev = std::env::var(env_name);
    if dev.is_ok() && dev.unwrap() == "true" {
        load(_filenames);
    }
}

/// Checks if an env variables already exists, if it exists it will be skipped
pub fn load(_filenames: Option<Vec<&str>>) {
    let filenames = filenames_or_default(_filenames);

    for i in filenames {
        let err = load_file(i, false);
        if err.is_some() {
            return;
        }
    }
}

/// Overwrites an env variable if it already exists
pub fn overload(_filenames: Option<Vec<&str>>) {
    let filenames = filenames_or_default(_filenames);

    for i in filenames {
        let err = load_file(i, true);
        if err.is_some() {
            return;
        }
    }
}

fn filenames_or_default(_filenames: Option<Vec<&str>>) -> Vec<&str> {
    return _filenames.unwrap_or(Vec::from([".env"]));
}

fn load_file(filename: &str, overload: bool) -> Option<io::Error> {
    let env_file = EnvFile::new(&Path::new(filename));
    if env_file.is_err() {
        return env_file.err();
    }

    let mut current_env: HashMap<String, bool> = HashMap::new();
    for e in std::env::vars() {
        current_env.insert(e.0, true);
    }
    for (key, value) in env_file.unwrap().store {
        if !current_env.contains_key(&*key) || overload {
            std::env::set_var(key, value);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    use crate::load;

    fn env_is_set(key: &str, value: &str) -> bool {
        match std::env::var(key) {
            Ok(s) => s == value,
            _ => false
        }
    }

    #[test]
    fn it_works() {
        load(None);
        assert_eq!(env_is_set("MONGO_URI", "mongodb://admin:password@127.0.0.1:27017/?authSource=admin"), true);
        assert_eq!(env_is_set("MYSQL_URI", "mysql://admin:password@127.0.0.1:3306"), false);
    }
}
