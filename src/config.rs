use dirs;
use std::{io::Read, path::PathBuf};
use yaml_rust2::YamlLoader;

pub fn get_config_dir() -> std::path::PathBuf {
    let mut fallback: std::path::PathBuf = dirs::config_dir().unwrap();
    //  {
    //     Some(val) => val,
    //     None => panic!("Failed to Get env"),
    // };
    fallback.push("glab-cli");
    return fallback;
}

pub fn open_config() -> Vec<yaml_rust2::Yaml> {
    let mut config_dir: PathBuf = get_config_dir();
    config_dir.push("config.yml");

    let mut file = match std::fs::File::open(&config_dir) {
        Err(why) => panic!("couldn't open {}: {}", config_dir.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", config_dir.display(), why),
        Ok(_) => (),
    }

    return YamlLoader::load_from_str(&s).unwrap();
}

pub fn get_default_host() -> String {
    let config: Vec<yaml_rust2::Yaml> = open_config();
    let doc = &config[0];

    let ret: String = doc["host"]
        .as_str()
        .unwrap()
        .to_string()
        .lines()
        .collect::<String>();

    return ret;
}

pub fn get_hosts() {
    let config: Vec<yaml_rust2::Yaml> = open_config();
    let doc = &config[0];

    let hosts: Vec<yaml_rust2::Yaml> = match doc["hosts"].clone() {
        yaml_rust2::Yaml::Array(val) => val,
        _ => panic!("Not Array"),
    };

    for host in hosts {
        print!("{}", host.as_str().unwrap().to_string());
    }

    // return ret;
}
