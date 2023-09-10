use std::{
    format,
    fs::File,
    io::{Seek, Write},
    println,
};

use nmea::{Nmea, SentenceType};
use serialport::SerialPort;

use crate::{config::UBloxConfig, discard_timeout};

const BAUD_RATE: u32 = 38400;

pub struct Ublox {
    pub port: Box<dyn SerialPort>,
    pub nmea: Nmea,
    pub gga: String,
}

impl Ublox {
    pub fn print_device_list() {
        let ports = serialport::available_ports().unwrap();
        for port in ports {
            println!("---  Port: \"{}\"  ---", port.port_name);
            println!("{:?}", port.port_type);
            println!();
        }
    }

    pub fn connect(config: &UBloxConfig) -> Self {
        println!(
            "Opening serialport: {} baudrate: {}",
            config.serial_port, BAUD_RATE
        );
        let port = serialport::new(&config.serial_port, 38400).open().unwrap();
        Self {
            port,
            nmea: Nmea::default(),
            gga: String::new(),
        }
    }

    pub fn write_rtcm(&mut self, buf: &[u8]) {
        println!("UBlox: writing rtcm");
        self.port.write_all(buf).unwrap_or_else(discard_timeout);
    }

    pub fn read(&mut self, buf: &mut [u8]) {
        let len = self.port.read(buf).unwrap_or_else(discard_timeout);
        println!("UBlox: {} bytes read", len);
        for line in String::from_utf8_lossy(&buf[0..len]).split("\r\n") {
            if line.starts_with("$") {
                match self.nmea.parse(line) {
                    Ok(s_type) => {
                        if s_type == SentenceType::GGA {
                            self.gga = format!("{}\r\n", line);
                        }
                    }
                    Err(_) => (),
                }
            }
        }
    }

    pub fn write_out(&self, out_file: &mut File, append: bool) {
        let fix_time = self.nmea.fix_time.map_or("None".into(), |f| f.to_string());
        let fix_date = self.nmea.fix_date.map_or("None".into(), |f| f.to_string());
        let fix_type = self
            .nmea
            .fix_type
            .map_or("None".into(), |f| format!("{:?}", f));
        let lat = self.nmea.latitude.map_or("None".into(), |f| f.to_string());
        let lon = self.nmea.longitude.map_or("None".into(), |f| f.to_string());
        let alt = self.nmea.altitude.map_or("None".into(), |f| f.to_string());
        let speed = self
            .nmea
            .speed_over_ground
            .map_or("None".into(), |f| f.to_string());
        let course = self
            .nmea
            .true_course
            .map_or("None".into(), |f| f.to_string());
        let sattelites = self
            .nmea
            .num_of_fix_satellites
            .map_or("None".into(), |f| f.to_string());
        let hdop = self.nmea.hdop.map_or("None".into(), |f| f.to_string());
        let vdop = self.nmea.vdop.map_or("None".into(), |f| f.to_string());
        let pdop = self.nmea.pdop.map_or("None".into(), |f| f.to_string());
        let seperation = self
            .nmea
            .geoid_separation
            .map_or("None".into(), |f| f.to_string());

        let output = format!(
            " -- Ublox Data --\n\
            fix time: {}\n\
            fix date: {}\n\
            fix type: {}\n\
            lat: {}\n\
            lon: {}\n\
            alt: {}\n\
            speed: {}\n\
            course: {}\n\
            sattelites: {}\n\
            hdop: {}\n\
            vdop: {}\n\
            pdop: {}\n\
            separation: {}\n\n\
            ",
            fix_time,
            fix_date,
            fix_type,
            lat,
            lon,
            alt,
            speed,
            course,
            sattelites,
            hdop,
            vdop,
            pdop,
            seperation
        );
        if !append {
            out_file.seek(std::io::SeekFrom::Start(0)).unwrap();
            out_file.set_len(0).unwrap();
        }
        out_file.write_all(output.as_bytes()).unwrap();
    }
}
