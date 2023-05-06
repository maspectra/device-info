use crate::{
    core::builder::{IMainBuilder, MainDeviceInfoBuilder},
    plugins::{macos::plugin::IMacOSBuilder, windows::plugin::IWindowsBuilder},
};

mod core;
mod plugins;

fn main() {
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
    let serialized = serde_json::to_string(&builder).unwrap();
    println!("serialized = {}", serialized);
}
