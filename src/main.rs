use std::{
    fs::File,
    io::{Error, ErrorKind},
    panic, println,
    time::Duration,
};

use crate::{config::Config, ntrip::Ntrip, ublox::Ublox};

mod config;
mod ntrip;
mod ublox;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            "help" => print_help(),
            "config" => Config::generate_example_file(),
            "devices" => Ublox::print_device_list(),
            _ => {
                println!("Invalid Arguments!");
                print_help_info();
            }
        }
        return;
    }

    let config = Config::load();

    // configure out file
    println!(
        "Open out file: {} - append = {}",
        config.out_file, config.out_file_append
    );
    let mut out_file = File::create(config.out_file).unwrap();

    // One buffer for all io -> only one allocation
    let mut buf = [0; 4096];

    let mut ublox = Ublox::connect(&config.ublox);

    // Reading ublox until first valid gpgga
    // Connecting to ntrip caster with this gpgga
    let mut ntrip = loop {
        std::thread::sleep(Duration::from_secs(1));

        ublox.read(&mut buf);

        if !ublox.gga.is_empty() {
            break Ntrip::connect(&config.ntrip, &ublox.gga, &mut buf);
        }
    };

    ublox.write_out(&mut out_file, config.out_file_append);

    let mut send_gga = 5;

    loop {
        // Forward rtcm from ntrip caster
        let len = ntrip.read_rtcm(&mut buf);
        ublox.write_rtcm(&buf[0..len]);

        // Read ublox
        ublox.read(&mut buf);

        ublox.write_out(&mut out_file, config.out_file_append);

        // Send gpgga to ntrip caster every 5 secs
        if send_gga == 0 {
            ntrip.write_gpgga(&ublox.gga);
            send_gga = 5
        } else {
            send_gga -= 1;
        }
    }
}

pub fn discard_timeout<T: Default>(err: Error) -> T {
    if err.kind() == ErrorKind::TimedOut {
        println!("Operation Timed Out!");
        T::default()
    } else {
        panic!("{}", err)
    }
}

fn print_help_info() {
    println!(
        "Type \"{} help\" for information on usage!",
        env!("CARGO_PKG_NAME")
    )
}

fn print_help() {
    println!("Usage: {} <argument>", env!("CARGO_PKG_NAME"));
    println!();
    println!("argument:");
    println!("  help -> prints this info");
    println!("  config -> generates an example config file");
    println!("  devices -> prints a list of all available devices");
    println!();
    println!("No argument:");
    println!("  the programm will excecute according to the Config.toml file");
    println!();
}
