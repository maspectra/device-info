use std::{collections::HashMap, fmt};

use serde;

use crate::core::internal::{BaseDeviceInfoBuilder, IDeviceInfoBuilder};
#[cfg(target_os = "windows")]
use crate::core::string_tools::strip_trailing_newline;
#[cfg(target_os = "windows")]
use crate::plugins::windows::wmi::WmiSingleton;

#[allow(dead_code)]
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum WindowsBuilderComponents {
    LogonUserName,
    SystemDriveSerialNumber,
    MotherBoardSerialNumber,
    SystemUuid,
    MACAddress,
}

impl serde::Serialize for WindowsBuilderComponents {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl WindowsBuilderComponents {
    pub fn as_str(&self) -> &str {
        match *self {
            WindowsBuilderComponents::LogonUserName => "logonUserName",
            WindowsBuilderComponents::SystemDriveSerialNumber => "systemDriveSerialNumber",
            WindowsBuilderComponents::MotherBoardSerialNumber => "motherBoardSerialNumber",
            WindowsBuilderComponents::SystemUuid => "systemUuid",
            WindowsBuilderComponents::MACAddress => "MACAddress",
        }
    }
}
impl fmt::Display for WindowsBuilderComponents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub trait IWindowsBuilder: IDeviceInfoBuilder<WindowsBuilderComponents> {
    fn add_logon_user_name(&mut self) -> &mut Self;
    fn add_system_drive_serial_number(&mut self) -> &mut Self;
    fn add_mother_board_serial_number(&mut self) -> &mut Self;
    fn add_system_uuid(&mut self) -> &mut Self;
    fn add_mac_address(&mut self) -> &mut Self;
}

pub struct WindowsBuilder {
    _base: BaseDeviceInfoBuilder<WindowsBuilderComponents>,
}

impl WindowsBuilder {
    pub fn new() -> Self {
        Self {
            _base: BaseDeviceInfoBuilder::<WindowsBuilderComponents>::new(),
        }
    }
}

impl IDeviceInfoBuilder<WindowsBuilderComponents> for WindowsBuilder {
    fn get_components(&self) -> &HashMap<WindowsBuilderComponents, String> {
        &self._base.components
    }
    fn get_components_mut(&mut self) -> &mut HashMap<WindowsBuilderComponents, String> {
        &mut self._base.components
    }
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UserNameQueryResult {
    user_name: String,
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SerialNumberQueryResult {
    serial_number: String,
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct UUIDQueryResult {
    uuid: String,
}

#[cfg(target_os = "windows")]
#[derive(serde::Deserialize)]
#[allow(non_snake_case)]
struct MACAddressQueryResult {
    MACAddress: String,
}

impl IWindowsBuilder for WindowsBuilder {
    fn add_logon_user_name(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<UserNameQueryResult> =
                WmiSingleton::raw_query("SELECT UserName FROM Win32_ComputerSystem");

            self.add_component(
                &WindowsBuilderComponents::LogonUserName,
                strip_trailing_newline(res.get(0).expect("Nothing queried out").user_name.trim()),
            );
            self
        }

        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }
    fn add_system_drive_serial_number(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<SerialNumberQueryResult> =
                WmiSingleton::raw_query("SELECT SerialNumber FROM Win32_PhysicalMedia");

            self.add_component(
                &WindowsBuilderComponents::SystemDriveSerialNumber,
                strip_trailing_newline(
                    res.get(0)
                        .expect("Nothing queried out")
                        .serial_number
                        .trim(),
                ),
            );
            self
        }

        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_mother_board_serial_number(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<SerialNumberQueryResult> =
                WmiSingleton::raw_query("SELECT SerialNumber FROM Win32_BaseBoard");

            self.add_component(
                &WindowsBuilderComponents::MotherBoardSerialNumber,
                strip_trailing_newline(
                    res.get(0)
                        .expect("Nothing queried out")
                        .serial_number
                        .trim(),
                ),
            );
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_system_uuid(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<UUIDQueryResult> =
                WmiSingleton::raw_query("SELECT UUID FROM Win32_ComputerSystemProduct");

            self.add_component(
                &WindowsBuilderComponents::SystemUuid,
                strip_trailing_newline(res.get(0).expect("Nothing queried out").uuid.trim()),
            );
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }

    fn add_mac_address(&mut self) -> &mut Self {
        #[cfg(target_os = "windows")]
        {
            let res: Vec<MACAddressQueryResult> = WmiSingleton::raw_query(
                "SELECT MACAddress FROM Win32_NetworkAdapter WHERE MACAddress IS NOT NULL",
            );

            self.add_component(
                &WindowsBuilderComponents::MACAddress,
                strip_trailing_newline(res.get(0).expect("Nothing queried out").MACAddress.trim()),
            );
            self
        }
        #[cfg(not(target_os = "windows"))]
        {
            todo!()
        }
    }
}

impl Default for WindowsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[cfg(target_os = "windows")]
mod tests {
    use super::*;

    #[test]
    fn test_windows_builder_serial_number() {
        let mut builder = WindowsBuilder::new();
        builder.add_system_drive_serial_number();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_mother_board_serial_number() {
        let mut builder = WindowsBuilder::new();
        builder.add_mother_board_serial_number();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_system_uuid() {
        let mut builder = WindowsBuilder::new();
        builder.add_system_uuid();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_mac_address() {
        let mut builder = WindowsBuilder::new();
        builder.add_mac_address();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_logon_user_name() {
        let mut builder = WindowsBuilder::new();
        builder.add_logon_user_name();

        println!("{:?}", builder.get_components());
    }

    #[test]
    fn test_windows_builder_all() {
        let mut builder = WindowsBuilder::new();
        builder
            .add_logon_user_name()
            .add_system_drive_serial_number()
            .add_mother_board_serial_number()
            .add_system_uuid()
            .add_mac_address();

        println!("{:?}", builder.get_components());
    }
}
