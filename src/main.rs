use crate::{
    core::builder::{IMainBuilder, MainDeviceInfoBuilder},
    core::encryption::{encrypt, EncryptionAlgorithm},
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
fn do_encrypt(encrypt_m: &clap::ArgMatches) {
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

    let algorithm = match encrypt_m
        .get_one::<String>("algorithm")
        .map(|s| s.as_str())
        .unwrap()
    {
        "md5" => EncryptionAlgorithm::MD5,
        "sha1" => EncryptionAlgorithm::SHA1,
        "sha256" => EncryptionAlgorithm::SHA256,
        _ => panic!("Invalid algorithm"),
    };

    println!("{}", encrypt(builder, algorithm));
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
                .arg(
                    clap::Arg::new("algorithm")
                        .short('a')
                        .long("algorithm")
                        .aliases(["algo", "alg"])
                        .required(true)
                        .value_parser(["md5", "sha1", "sha256"])
                        .ignore_case(true)
                        .help("Encryption algorithm, support MD5, SHA1, SHA256"),
                )
                .after_help(
                    "Note:\n\
                    * Set Env ENCRYPTION_KEY_PREFIX=YOUR_SECRET to encrypt the machine code.\n\
                    * The program will collect partial device information and encrypt them into a machine code.\n\
                    * The machine code will be used to verify if the device matches with the machine code.",
                ),
        )
        .after_help(
            "Longer explanation to appear after the options when \
                 displaying the help information from --help or -h",
        );

    let m = cmd.try_get_matches().unwrap_or_else(|e| e.exit());

    match m.subcommand() {
        Some(("print", print_m)) => do_print(print_m),
        Some(("encrypt", encrypt_m)) => do_encrypt(encrypt_m),
        _ => panic!(),
    }
}
