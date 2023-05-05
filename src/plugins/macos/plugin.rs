use crate::core::string_tools::strip_trailing_newline;
use std::{collections::HashMap, process::Command};

pub struct MacOSBuilderPlugin {
    pub components: HashMap<String, String>,
}

impl MacOSBuilderPlugin {
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

        self.components.insert(
            "SystemDriveSerialNumber".to_string(),
            strip_trailing_newline(&output).to_owned(),
        );
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

        self.components.insert(
            "PlatformSerialNumber".to_string(),
            strip_trailing_newline(&output).to_owned(),
        );
        self
    }

    #[cfg(not(target_os = "macos"))]
    pub fn add_platform_serial_number(&mut self) -> &mut Self {
        todo!()
    }
}
