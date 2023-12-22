use std::io::Error;

use crate::{
    core::builder::{IMainBuilder, MainDeviceInfoBuilder},
    core::crypto,
    core::internal::IDeviceInfoBuilder,
    plugins::{macos::plugin::IMacOSBuilder, windows::plugin::IWindowsBuilder},
};

mod core;
mod plugins;

/// Print device information
///
/// Either print in json or text format
fn do_print(print_m: &clap::ArgMatches) {
    let mut builder = MainDeviceInfoBuilder::new();
    builder
        .add_user_name()
        .add_platform_name()
        .add_device_name()
        .add_cpu_arch()
        .add_os_distro()
        .on_windows(|windows_builder| {
            windows_builder
                .add_logon_user_name()
                .add_system_drive_serial_number()
                .add_mother_board_serial_number()
                .add_system_uuid()
                .add_mac_address()
                .add_processor_id()
                .add_machine_guid()
        })
        .on_macos(|macos_builder| {
            macos_builder
                .add_platform_serial_number()
                .add_system_drive_serial_number()
        });

    match print_m
        .get_one::<String>("format")
        .map(|s| s.as_str())
        .unwrap()
    {
        "json" => println!("{}", serde_json::to_string(&builder).unwrap()),
        "text" => println!("{}", builder),
        _ => panic!("Invalid format"),
    }
}

/// Encrypt specific device information into a machine code
///
/// *NOTE*: It will collect different device information on different platforms
///
/// On **Windows**, it will collect LogonUserName + SystemUuid + MotherBoardSerialNumber + SystemDriveSerialNumber
/// On **MacOS**, it will collect UserName + PlatformSerialNumber + SystemDriveSerialNumber
fn do_encrypt() {
    let mut builder = MainDeviceInfoBuilder::new();
    builder
        .on_windows(|windows_builder| {
            windows_builder
                .add_logon_user_name()
                .add_system_drive_serial_number()
                .add_mother_board_serial_number()
                .add_system_uuid()
        })
        .on_macos(|macos_builder| {
            macos_builder
                .add_platform_serial_number()
                .add_system_drive_serial_number()
        });

    // add username if not windows
    if !cfg!(target_os = "windows") {
        builder.add_user_name();
    }

    let key = crypto::aes::generate_key();

    match crypto::aes::encrypt(&key, serde_json::to_string(&builder).unwrap().as_str()) {
        Ok(encrypted) => println!("{}", encrypted),
        Err(e) => panic!("{}", e),
    }
}

fn do_check(check_m: &clap::ArgMatches) -> Result<(), Error> {
    let key = crypto::aes::generate_key();
    let code = check_m.get_one::<String>("code").unwrap();

    match crypto::aes::decrypt(&key, code) {
        Ok(decrypted) => {
            let mut builder = MainDeviceInfoBuilder::new();
            builder
                .on_windows(|windows_builder| {
                    windows_builder
                        .add_logon_user_name()
                        .add_system_drive_serial_number()
                        .add_mother_board_serial_number()
                        .add_system_uuid()
                })
                .on_macos(|macos_builder| {
                    macos_builder
                        .add_platform_serial_number()
                        .add_system_drive_serial_number()
                });

            // add username if not windows
            if !cfg!(target_os = "windows") {
                builder.add_user_name();
            }
            println!("{}", decrypted);
            let deserialized: MainDeviceInfoBuilder = match serde_json::from_str(&decrypted) {
                Ok(v) => v,
                Err(_) => panic!(),
            };

            // Compare components one by one
            for (k, v) in builder.get_components() {
                if deserialized.get_components().get(k) != Some(v) {
                    panic!();
                }
            }

            Ok(())
        }
        Err(_) => panic!(),
    }
}

fn main() {
    let cmd = clap::Command::new(env!("CARGO_PKG_NAME"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .about("Device information CLI tool")
        .subcommand(
            clap::Command::new("print")
                .about("Print device information")
                .arg(
                    clap::Arg::new("format")
                        .short('f')
                        .long("format")
                        .alias("fmt")
                        .default_value("text")
                        .value_parser(["json", "text"])
                        .ignore_case(true)
                        .help("output format, default json, support json, text"),
                ),
        )
        .subcommand(
            clap::Command::new("encrypt")
                .about("Encrypt specific device information into a machine code")
                .after_help(
                    "Note:\n\
                    * Set Env ENCRYPTION_KEY=YOUR_SECRET to encrypt the machine code.\n\
                    * The program will collect partial device information and encrypt them into a machine code.\n\
                    * The machine code will be used to verify if the device matches with the machine code.",
                ),
        )
        .subcommand(
            clap::Command::new("check")
                .about("Check if the device matches with the machine code")
                .arg(
                    clap::Arg::new("code")
                        .help("Code generated by encrypt command")
                        .required(true)
                        .index(1),
                ),
        )
        .after_help(
            "This program will collect partial device information and encrypt them into a machine code.\n"
        );

    let m = cmd.try_get_matches().unwrap_or_else(|e| e.exit());

    match m.subcommand() {
        Some(("print", print_m)) => do_print(print_m),
        Some(("encrypt", _)) => do_encrypt(),
        Some(("check", check_m)) => match do_check(check_m) {
            Ok(_) => println!("0"),
            Err(_) => println!("1"),
        },
        _ => panic!(),
    }
}
