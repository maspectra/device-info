#[cfg(target_os = "macos")]
use crate::core::string_tools::strip_trailing_newline;
#[cfg(target_os = "macos")]
use std::process::Command;

use std::collections::HashMap;

use crate::core::internal::IDeviceInfoBuilder;

pub struct MacOSBuilder {
    pub components: HashMap<String, String>,
}

impl IDeviceInfoBuilder for MacOSBuilder {
    fn get_components(&self) -> &HashMap<String, String> {
        &self.components
    }

    fn get_components_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.components
    }
}

impl MacOSBuilder {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    #[cfg(target_os = "macos")]
    pub fn add_system_drive_serial_number(&mut self) -> &mut Self {
        let cmd = Command::new("sh")
            .arg("-c")
            .arg("system_profiler SPNVMeDataType | sed -En 's/.*Serial Number: ([\\d\\w]*)//p'")
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8(cmd.stdout).expect("failed to decode output");

        self.add_component("systemDriveSerialNumber", strip_trailing_newline(&output));
        self
    }

    #[cfg(not(target_os = "macos"))]
    pub fn add_system_drive_serial_number(&mut self) -> &mut Self {
        todo!()
    }

    #[cfg(target_os = "macos")]
    pub fn add_platform_serial_number(&mut self) -> &mut Self {
        let cmd = Command::new("sh")
            .arg("-c")
            .arg("ioreg -l | grep IOPlatformSerialNumber | sed 's/.*= //' | sed 's/\"//g'")
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8(cmd.stdout).expect("failed to decode output");

        self.add_component("platformSerialNumber", strip_trailing_newline(&output));
        self
    }

    #[cfg(not(target_os = "macos"))]
    pub fn add_platform_serial_number(&mut self) -> &mut Self {
        todo!()
    }
}
