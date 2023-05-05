#[cfg(target_os = "macos")]
use std::process::Command;
use std::{collections::HashMap, fmt};

use serde::Serialize;

use crate::core::internal::{BaseDeviceInfoBuilder, IDeviceInfoBuilder};
#[cfg(target_os = "macos")]
use crate::core::string_tools::strip_trailing_newline;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum MacOSBuilderComponents {
    SystemDriveSerialNumber,
    PlatformSerialNumber,
}

impl Serialize for MacOSBuilderComponents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl MacOSBuilderComponents {
    pub fn as_str(&self) -> &str {
        match *self {
            MacOSBuilderComponents::SystemDriveSerialNumber => "systemDriveSerialNumber",
            MacOSBuilderComponents::PlatformSerialNumber => "platformSerialNumber",
        }
    }
}
impl fmt::Display for MacOSBuilderComponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub trait IMacOSBuilder: IDeviceInfoBuilder<MacOSBuilderComponents> {
    fn add_system_drive_serial_number(&mut self) -> &mut Self;
    fn add_platform_serial_number(&mut self) -> &mut Self;
}

pub struct MacOSBuilder {
    _base: BaseDeviceInfoBuilder<MacOSBuilderComponents>,
}

impl MacOSBuilder {
    pub fn new() -> Self {
        Self {
            _base: BaseDeviceInfoBuilder::<MacOSBuilderComponents>::new(),
        }
    }
}

impl IDeviceInfoBuilder<MacOSBuilderComponents> for MacOSBuilder {
    fn get_components(&self) -> &HashMap<MacOSBuilderComponents, String> {
        &self._base.components
    }
    fn get_components_mut(&mut self) -> &mut HashMap<MacOSBuilderComponents, String> {
        &mut self._base.components
    }
}

impl IMacOSBuilder for MacOSBuilder {
    fn add_system_drive_serial_number(&mut self) -> &mut Self {
        #[cfg(target_os = "macos")]
        {
            let cmd = Command::new("sh")
                .arg("-c")
                .arg("system_profiler SPNVMeDataType | sed -En 's/.*Serial Number: ([\\d\\w]*)//p'")
                .output()
                .expect("failed to execute process");

            let output = String::from_utf8(cmd.stdout).expect("failed to decode output");

            self.add_component(
                &MacOSBuilderComponents::SystemDriveSerialNumber,
                strip_trailing_newline(&output),
            );
            self
        }

        #[cfg(not(target_os = "macos"))]
        {
            todo!()
        }
    }

    fn add_platform_serial_number(&mut self) -> &mut Self {
        #[cfg(target_os = "macos")]
        {
            let cmd = Command::new("sh")
                .arg("-c")
                .arg("ioreg -l | grep IOPlatformSerialNumber | sed 's/.*= //' | sed 's/\"//g'")
                .output()
                .expect("failed to execute process");

            let output = String::from_utf8(cmd.stdout).expect("failed to decode output");

            self.add_component(
                &MacOSBuilderComponents::PlatformSerialNumber,
                strip_trailing_newline(&output),
            );
            self
        }
        #[cfg(not(target_os = "macos"))]
        {
            todo!()
        }
    }
}

impl Default for MacOSBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(target_os = "macos")]
mod tests {
    use super::*;

    #[test]
    fn test_macos_builder() {
        let mut builder = MacOSBuilder::new();
        builder.add_system_drive_serial_number();
        builder.add_platform_serial_number();

        let components = builder.get_components();
        assert!(components
            .get(&MacOSBuilderComponents::SystemDriveSerialNumber)
            .is_some());
        assert!(components
            .get(&MacOSBuilderComponents::PlatformSerialNumber)
            .is_some());
    }
}
