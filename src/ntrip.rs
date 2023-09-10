use std::{
    format,
    io::{Read, Write},
    net::TcpStream,
    println,
};

use base64::{engine::general_purpose::STANDARD, Engine};

use crate::config::NtripConfig;

pub struct Ntrip {
    stream: TcpStream,
}

impl Ntrip {
    pub fn connect(config: &NtripConfig, gga: &str, buf: &mut [u8]) -> Self {
        // Connecting to the server
        println!("Ntrip connect to:");
        println!("  url: {}", config.url);
        println!("  mountpoint: {}", config.mountpoint);
        println!("  username: {}", config.username);
        println!("  password: {}\n", config.password);
        let mut stream = TcpStream::connect(&config.url).unwrap();
        // Write Request
        let request = get_rev2(config, gga);
        stream.write_all(&request).unwrap();
        // Answer
        let len = stream.read(buf).unwrap();
        println!(
            " --- Html answer --- \n{}",
            String::from_utf8_lossy(&buf[0..len])
        );

        Self { stream }
    }

    pub fn write_gpgga(&mut self, gga: &String) {
        println!("Ntrip: writing gga");
        self.stream.write_all(gga.as_bytes()).unwrap()
    }

    pub fn read_rtcm(&mut self, buf: &mut [u8]) -> usize {
        let len = self.stream.read(buf).unwrap();
        println!("Ntrip: {} rtcm bytes read", len);
        len
    }
}

fn get_rev2(config: &NtripConfig, gga: &str) -> Vec<u8> {
    let credentials = format!("{}:{}", config.username, config.password);
    let encoded_credentials = STANDARD.encode(credentials);

    // See https://www.use-snip.com/kb/knowledge-base/ntrip-rev1-versus-rev2-formats/
    format!(
        "GET /{} HTTP/1.1\r\n\
        Host: {}\r\n\
        Ntrip-Version: Ntrip/2.0\r\n\
        User-Agent: NTRIP RustClient\r\n\
        Authorization: Basic {}\r\n\
        {}\r\n\
        \r\n",
        config.mountpoint, config.url, encoded_credentials, gga
    )
    .into_bytes()
}
