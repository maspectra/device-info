use crate::{
    core::builder::{IMainBuilder, MainDeviceInfoBuilder},
    plugins::macos::plugin::IMacOSBuilder,
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
        .on_macos(|macos_builder| {
            macos_builder
                .add_platform_serial_number()
                .add_system_drive_serial_number()
        });
    let serialized = serde_json::to_string(&builder).unwrap();
    println!("serialized = {}", serialized);
}
