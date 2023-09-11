# rust-rtk
A simple programm which connects to an ntrip caster and to a ublox device and sends
 - nmea bytes from ublox to ntrip
 - rtcm bytes from ntrip to ublox

# compiling
To compile the project you need the rust package manager cargo(https://www.rust-lang.org/).
```bash
cargo build --release
```
The output binary is at ./target/release/rtk. This produces a standalone binary, thus you can copy it anywhere.

# Run
Look at the help output with:
```bash
rtk help
```
