use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NtripConfig {
    pub url: String,
    pub mountpoint: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UBloxConfig {
    pub serial_port: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub out_file: String,
    pub out_file_append: bool,
    pub ntrip: NtripConfig,
    pub ublox: UBloxConfig,
}

impl Config {
    pub fn load() -> Self {
        let bytes = fs::read("Config.toml").unwrap();
        let string = String::from_utf8(bytes).unwrap();
        let config = toml::from_str(&string).unwrap();
        config
    }

    pub fn generate_example_file() {
        let example = r#"# example log file

out_file = "out_file.txt"

# true -> data will fill up the file
# false -> data will reaplace old data
out_file_append = false

[ntrip]
url = "rtk2go.com:2101"
mountpoint = "NEAR_DEU"
username = "your@email.com"
password = "None"

[ublox]
# needs to run in sudo
serial_port = "/dev/ttyUSB0"
"#;
        fs::write("Config.toml", example).unwrap();
    }
}
