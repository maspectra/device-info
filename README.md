# device-info

This is a Rust implementation to obtain device information. It is heavily inspired by C# library [DeviceId](https://github.com/MatthewKing/DeviceId).

## Usage

### Lib

```rust
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
```

### Command Line

```bash
device-info --help
```
